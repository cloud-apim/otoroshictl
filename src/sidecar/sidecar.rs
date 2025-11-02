use std::sync::Arc;

use crate::cli::cliopts::CliOpts;
use crate::{cli_stdout_printline, cli_stderr_printline};

use super::config::OtoroshiSidecarConfig;
use super::cache::SidecarCache;

use run_script::ScriptOptions;

pub struct Sidecar {}

impl Sidecar {

    fn update_cache(cache: Arc<SidecarCache>) -> () {
        tokio::spawn(async move {
            loop {
                cache.update().await;
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            }
        });
    }

    pub fn how_to() -> () {
        let mut logger = paris::Logger::new();

        logger.log("");
        logger.log("In order to run the sidecar with transparent proxying of the mesh calls, you need to create a user. This will be used to avoid being intercepted by the iptables rules.");
        logger.log("");
        logger.log("  <green>useradd -u 5678 -U otoroshictl</>");
        logger.log("");
        logger.log("Then you will have to install iptables rules to route traffic through the sidecar proxies: ");
        logger.log("");
        logger.log("  <green>sudo otoroshictl sidecar install --user otoroshictl -f ./sidecar.yaml</>");
        logger.log("");
        logger.log("This command will add several rules to your local iptables permanently. Remember that you can uninstall the iptables rules at any time with the following command: ");
        logger.log("");
        logger.warn("  <red>sudo otoroshictl sidecar uninstall</>");
        logger.log("");
        logger.log("Then you can run your sidecar with the following command: ");
        logger.log("");
        logger.log("  <green>runuser -u otoroshictl -- otoroshictl sidecar run -f ./sidecar.yaml</>");
        logger.log("");
    }

    pub async fn start(clip_opts: CliOpts, sidecar_config: OtoroshiSidecarConfig, dns_port: &Option<u16>) -> () {
        if sidecar_config.clone().spec.inbound.tls.map(|i| i.enabled).unwrap_or(false) {
            Self::start_inbound_https(clip_opts, sidecar_config, dns_port).await;
        } else {
            Self::start_inbound_http(clip_opts, sidecar_config, dns_port).await;
        }
    }

    pub async fn start_inbound_http(clip_opts: CliOpts, sidecar_config: OtoroshiSidecarConfig, dns_port: &Option<u16>) -> () {
        
        let cache = Arc::new(SidecarCache::new(sidecar_config.clone(), clip_opts.clone()));

        Self::update_cache(cache.clone());

        let dns = crate::sidecar::dns::DnsServer::start(dns_port.clone(), sidecar_config.clone());
        let outbound = crate::sidecar::outboundproxy::OutboundProxy::start(clip_opts.clone(), sidecar_config.clone(), cache.clone());
        let inbound = crate::sidecar::inboundproxy::InboundProxy::start_http(sidecar_config.clone(), cache.clone());
        
        let _ = futures::join!(dns, outbound, inbound);
    }

    pub async fn start_inbound_https(clip_opts: CliOpts, sidecar_config: OtoroshiSidecarConfig, dns_port: &Option<u16>) -> () {
        
        let cache = Arc::new(SidecarCache::new(sidecar_config.clone(), clip_opts.clone()));

        Self::update_cache(cache.clone());

        let dns = crate::sidecar::dns::DnsServer::start(dns_port.clone(), sidecar_config.clone());
        let outbound = crate::sidecar::outboundproxy::OutboundProxy::start(clip_opts.clone(), sidecar_config.clone(), cache.clone());
        let inbound = crate::sidecar::inboundproxy::InboundProxy::start_https(sidecar_config.clone(), cache.clone());
        
        let _ = futures::join!(dns, outbound, inbound);
    }

    fn get_backup_file_path() -> String {
        confy::get_configuration_file_path("io.otoroshi.otoroshictl", Some("iptables_backup")).unwrap().to_string_lossy().to_string()
    }

    pub fn install(sidecar_config: OtoroshiSidecarConfig, user: &String, dry: &Option<bool>) -> () {

        let outbound_port = sidecar_config.spec.outbounds.port.unwrap_or(15001);
        let inbound_port = sidecar_config.spec.inbound.port.unwrap_or(15000);
        let target_port = sidecar_config.spec.inbound.target_port.unwrap_or(8080);
        let dns_port = sidecar_config.spec.dns_port.unwrap_or(15053);

        let out = std::process::Command::new("id")
            .arg("-u")
            .arg(user)
            .output().unwrap().stdout;
        let uid: i32 = String::from_utf8(out).unwrap().trim_end_matches('\n').to_string().parse().unwrap();

        // --------- cli_stdout_printline!("useradd -u 5678 -U otoroshictl");
        // --------- cli_stdout_printline!("iptables -t nat -A OUTPUT -p tcp --dport {} -j DNAT --to-destination 127.0.0.1:{}", 80, outbound_port);
        // --------- cli_stdout_printline!("iptables -t nat -A INPUT -p tcp --dport {} -j DNAT --to-destination 127.0.0.1:{}", target_port, inbound_port);
        // --------- cli_stdout_printline!("iptables -t nat -A OUTPUT -p udp --dport 53 -j DNAT --to-destination 127.0.0.1:{}", dns_port);

        let iptables_backup = format!("iptables-save -f '{}'", Self::get_backup_file_path());

        // create user
        // let user_add = format!("useradd -u 5678 -U otoroshictl");

        // outbound rules: call to anything 80 to local outbound proxy but not for otoroshictl user
        let outbound_rules = format!("
iptables -t nat -N OTOCTL_OUTBOUND_REDIRECT
iptables -t nat -I OUTPUT 1 -p tcp --dport 80 -m owner --uid-owner {} -j RETURN  
iptables -t nat -A OUTPUT -p tcp --dport 80 -j OTOCTL_OUTBOUND_REDIRECT 
iptables -t nat -A OTOCTL_OUTBOUND_REDIRECT -p tcp -j REDIRECT --to-ports {}", uid, outbound_port);

        // inbound rules: call to backend port goes to local inbound proxy
        let inbound_rule = format!("
iptables -t nat -N OTOCTL_INBOUND_REDIRECT
iptables -t nat -A INPUT -p tcp --dport {} -j OTOCTL_INBOUND_REDIRECT
iptables -t nat -A OTOCTL_INBOUND_REDIRECT -p tcp -j REDIRECT --to-ports {}", target_port, inbound_port);

        // dns rules, call udp to 53 goes to local dns server but not for the otoroshictl user
        let dns_rule = format!("
iptables -t nat -N OTOCTL_DNS_REDIRECT
iptables -t nat -I OUTPUT 1 -p udp --dport 53 -m owner --uid-owner {} -j RETURN
iptables -t nat -A OUTPUT -p udp --dport 53 -j OTOCTL_DNS_REDIRECT
iptables -t nat -A OTOCTL_DNS_REDIRECT -p udp -j REDIRECT --to-ports {}", uid, dns_port);

        let script = format!("{}\n{}\n{}\n{}\niptables -t nat --list\n", iptables_backup, outbound_rules, inbound_rule, dns_rule);

        if dry.clone().unwrap_or(false) {
            cli_stdout_printline!("{}", script);
            std::process::exit(0);
        } else {
            let options = ScriptOptions::new();
            let args = vec![];
            let (code, output, error) = run_script::run(
                &script,
                &args,
                &options,
            )
            .unwrap();
            if code == 0 {
                cli_stdout_printline!("{}", output);
                std::process::exit(0);
            } else {
                cli_stdout_printline!("{}", output);
                cli_stderr_printline!("{}", error);
                std::process::exit(-1);
            }
        }
    }

    pub fn uninstall(dry: &Option<bool>) -> () {
        let iptables_cleanup = "
iptables -P INPUT ACCEPT
iptables -P FORWARD ACCEPT
iptables -P OUTPUT ACCEPT
iptables -F
iptables -X
iptables -Z 
iptables -t nat -F
iptables -t nat -X
iptables -t mangle -F
iptables -t mangle -X
iptables -t raw -F
iptables -t raw -X";
        let iptables_restore = format!("iptables-restore '{}'", Self::get_backup_file_path());
        let script = format!("{}\n{}\niptables -t nat --list\n", iptables_cleanup, iptables_restore);

        if dry.clone().unwrap_or(false) {
            cli_stdout_printline!("{}", script);
            std::process::exit(0);
        } else {
            let options = ScriptOptions::new();
            let args = vec![];
            let (code, output, error) = run_script::run(
                &script,
                &args,
                &options,
            )
            .unwrap();
            if code == 0 {
                cli_stdout_printline!("{}", output);
                std::process::exit(0);
            } else {
                cli_stdout_printline!("{}", output);
                cli_stderr_printline!("{}", error);
                std::process::exit(-1);
            }
        }
    }
}