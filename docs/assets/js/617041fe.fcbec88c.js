"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[199],{3245:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>a,contentTitle:()=>c,default:()=>h,frontMatter:()=>r,metadata:()=>l,toc:()=>u});var o=n(4848),s=n(8453),i=n(9229);const r={sidebar_position:3},c="Setup",l={id:"setup",title:"Setup",description:"otoroshictl is capable of managing multiple otoroshi clusters with multiple users, but the first thing we need to make is to create the otoroshictl config file.",source:"@site/docs/setup.mdx",sourceDirName:".",slug:"/setup",permalink:"/otoroshictl/docs/setup",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Install",permalink:"/otoroshictl/docs/install"},next:{title:"Usage",permalink:"/otoroshictl/docs/category/usage"}},a={},u=[{value:"Create a new cluster",id:"create-a-new-cluster",level:2},{value:"Change the current config.",id:"change-the-current-config",level:2},{value:"Modify an existing cluster",id:"modify-an-existing-cluster",level:2},{value:"All possible config. subcommands",id:"all-possible-config-subcommands",level:2}];function d(t){const e={code:"code",h1:"h1",h2:"h2",p:"p",...(0,s.R)(),...t.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(e.h1,{id:"setup",children:"Setup"}),"\n",(0,o.jsxs)(e.p,{children:[(0,o.jsx)(e.code,{children:"otoroshictl"})," is capable of managing multiple otoroshi clusters with multiple users, but the first thing we need to make is to create the ",(0,o.jsx)(e.code,{children:"otoroshictl"})," config file."]}),"\n",(0,o.jsx)(e.p,{children:"To do that, try"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config reset"}),"\n",(0,o.jsx)(e.p,{children:"then you'll be able to list the otoroshi cluster you can manage with"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config list",result:"\n\n+---------+---------+------------+\n| name    | current | cloud_apim |\n+---------+---------+------------+\n| default | yes     |            |\n+---------+---------+------------+\n"}),"\n",(0,o.jsx)(e.p,{children:"and display the current one"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config current-config",result:"\n\n---\napiVersion: v1\nkind: OtoroshiCtlConfig\nmetadata: {}\ncloud_apim: ~\nusers:\n  - name: default\n    client_id: admin-api-apikey-id\n    client_secret: admin-api-apikey-secret\n    health_key: ~\ncontexts:\n  - name: default\n    cluster: default\n    user: default\n    cloud_apim: false\nclusters:\n  - name: default\n    hostname: otoroshi-api.oto.tools\n    ip_addresses: ~\n    port: 8080\n    tls: false\n    client_cert: ~\n    routing_hostname: ~\n    routing_port: ~\n    routing_tls: ~\n    routing_ip_addresses: ~\ncurrent_context: default\n"}),"\n",(0,o.jsx)(e.p,{children:"by default the registered otoroshi cluster is supposed to be local and use default credentials, but you can modify it of even create a new one"}),"\n",(0,o.jsx)(e.h2,{id:"create-a-new-cluster",children:"Create a new cluster"}),"\n",(0,o.jsx)(e.p,{children:"to create a new cluster configuration just do the following"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config add new-cluster --hostname otoroshi.foo.bar --port 8443 --tls --client-id xxx --client-secret xxxxx"}),"\n",(0,o.jsxs)(e.p,{children:["you can even add ",(0,o.jsx)(e.code,{children:"--current"})," to make it the current one"]}),"\n",(0,o.jsx)(e.p,{children:"now if you list your clusters you have"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config list",result:"\n\n+-------------+---------+------------+\n| name        | current | cloud_apim |\n+-------------+---------+------------+\n| default     |         |            |\n+-------------+---------+------------+\n| new-cluster | yes     |            |\n+-------------+---------+------------+\n"}),"\n",(0,o.jsx)(e.p,{children:"with the content"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config current-config",result:"\n\n---\napiVersion: v1\nkind: OtoroshiCtlConfig\nmetadata: {}\ncloud_apim: ~\nusers:\n  - name: default\n    client_id: admin-api-apikey-id\n    client_secret: admin-api-apikey-secret\n    health_key: ~\n  - name: new-cluster\n    client_id: xxx\n    client_secret: xxxxx\n    health_key: ~\ncontexts:\n  - name: default\n    cluster: default\n    user: default\n    cloud_apim: false\n  - name: new-cluster\n    cluster: new-cluster\n    user: new-cluster\n    cloud_apim: false\nclusters:\n  - name: default\n    hostname: otoroshi-api.oto.tools\n    ip_addresses: ~\n    port: 8080\n    tls: false\n    client_cert: ~\n    routing_hostname: ~\n    routing_port: ~\n    routing_tls: ~\n    routing_ip_addresses: ~\n  - name: new-cluster\n    hostname: otoroshi.foo.bar\n    ip_addresses: ~\n    port: 8443\n    tls: true\n    client_cert: ~\n    routing_hostname: ~\n    routing_port: ~\n    routing_tls: false\n    routing_ip_addresses: ~\ncurrent_context: new-cluster\n"}),"\n",(0,o.jsx)(e.h2,{id:"change-the-current-config",children:"Change the current config."}),"\n",(0,o.jsxs)(e.p,{children:["you can change the current config at any moment using the ",(0,o.jsx)(e.code,{children:"use"})," command"]}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config list",result:"\n\n+-------------+---------+------------+\n| name        | current | cloud_apim |\n+-------------+---------+------------+\n| default     | yes     |            |\n+-------------+---------+------------+\n| new-cluster |         |            |\n+-------------+---------+------------+\n\n$ otoroshictl config use new-cluster\n$ otoroshictl config list\n\n+-------------+---------+------------+\n| name        | current | cloud_apim |\n+-------------+---------+------------+\n| default     |         |            |\n+-------------+---------+------------+\n| new-cluster | yes     |            |\n+-------------+---------+------------+\n\n"}),"\n",(0,o.jsx)(e.h2,{id:"modify-an-existing-cluster",children:"Modify an existing cluster"}),"\n",(0,o.jsxs)(e.p,{children:["you can also change an existing configuration with the commands ",(0,o.jsx)(e.code,{children:"set-cluster"}),", ",(0,o.jsx)(e.code,{children:"set-user"}),", ",(0,o.jsx)(e.code,{children:"set-context"})]}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config set-cluster new-cluster --hostname otoroshi.bar.foo --port 8080 --tls false\n$ otoroshictl config set-user new-cluster --client-id yyy ---client-id yyyyyyy"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config current-config",result:"\n\n---\napiVersion: v1\nkind: OtoroshiCtlConfig\nmetadata: {}\ncloud_apim: ~\nusers:\n  - name: default\n    client_id: admin-api-apikey-id\n    client_secret: admin-api-apikey-secret\n    health_key: ~\n  - name: new-cluster\n    client_id: yyy\n    client_secret: yyyyyyy\n    health_key: ~\ncontexts:\n  - name: default\n    cluster: default\n    user: default\n    cloud_apim: false\n  - name: new-cluster\n    cluster: new-cluster\n    user: new-cluster\n    cloud_apim: false\nclusters:\n  - name: default\n    hostname: otoroshi-api.oto.tools\n    ip_addresses: ~\n    port: 8080\n    tls: false\n    client_cert: ~\n    routing_hostname: ~\n    routing_port: ~\n    routing_tls: ~\n    routing_ip_addresses: ~\n  - name: new-cluster\n    hostname: otoroshi.bar.foo\n    ip_addresses: ~\n    port: 8080\n    tls: false\n    client_cert: ~\n    routing_hostname: ~\n    routing_port: ~\n    routing_tls: false\n    routing_ip_addresses: ~\ncurrent_context: new-cluster\n"}),"\n",(0,o.jsx)(e.h2,{id:"all-possible-config-subcommands",children:"All possible config. subcommands"}),"\n",(0,o.jsx)(i.A,{command:"otoroshictl config -h",result:"\n\nManage all the otoroshi cluster configurations you want to connect to with otoroshictl\n\nUsage: otoroshictl config [OPTIONS] <COMMAND>\n\nCommands:\n  current-config       Display the current config. file content\n  edit-current-config  Edit the current config. file\n  current-location     Display current config. location\n  current-context      Display current context\n  use-context          Set the current context\n  use                  Set the current context\n  rename-context       Rename a context\n  list                 Display the list of usable contexts\n  list-clusters        Display the list of clusters\n  list-users           Display the list of users\n  list-contexts        Display the list of contexts\n  set-cluster          Create or update a cluster\n  set-user             Create or update a user\n  set-context          Create or update a context\n  add                  Create and set a full config\n  delete-cluster       Delete a cluster\n  delete-user          Delete a user\n  delete-context       Delete a context\n  delete               Delete a full context with the associated cluster and user\n  reset                Delete configuration and start with a clean one\n  import               Import a context file with current context file\n  help                 Print this message or the help of the given subcommand(s)\n\nOptions:\n  -v, --verbose\n          Turn debugging information on\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -c, --config-file <FILE or URL>\n          Sets a custom config file\n      --otoroshi-cluster-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-id <CLIENT_ID>\n          Sets the client_id to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-secret <CLIENT_SECRET>\n          Sets the client_secret to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-health-key <HEALTH_KEY>\n          Sets the health_key to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-cert-location <FILE>\n          Sets the client cert location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-key-location <FILE>\n          Sets the client cert key location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-ca-location <FILE>\n          Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file\n  -h, --help\n          Print help\n"})]})}function h(t={}){const{wrapper:e}={...(0,s.R)(),...t.components};return e?(0,o.jsx)(e,{...t,children:(0,o.jsx)(d,{...t})}):d(t)}},9229:(t,e,n)=>{n.d(e,{A:()=>i});var o=n(6540),s=n(4848);class i extends o.Component{state={copy:!1};copy=()=>{this.setState({copy:!0},(()=>{navigator.clipboard.writeText(this.props.command),setTimeout((()=>{this.setState({copy:!1})}),1e3)}))};render(){const t=this.state.copy?"rgb(40, 167, 69)":"white";let e=(this.props.command||"").trim();return e.startsWith("\n")&&(e=e.substring(1)),e.startsWith("$ ")&&(e=e.substring(2)),(0,s.jsx)("div",{className:"terminal-component",style:{width:"100%",marginTop:20},children:(0,s.jsxs)("div",{style:{maxWidth:"100%",display:"flex",flexDirection:"column",borderRadius:5},children:[(0,s.jsxs)("div",{style:{borderTopLeftRadius:5,borderTopRightRadius:5,height:"2rem",gap:".35rem",display:"flex",flexDirection:"row",justifyContent:"center",alignItems:"center",background:"#3f52e3",color:"white"},children:[(0,s.jsx)("div",{style:{marginLeft:".35rem",width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("span",{style:{width:"100%"}}),(0,s.jsx)("div",{style:{cursor:"pointer",width:30,height:30},onClick:this.copy,children:(0,s.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",fill:"none",viewBox:"0 0 24 24","stroke-width":"1.5",stroke:t,className:"w-5 h-5",children:(0,s.jsx)("path",{"stroke-linecap":"round","stroke-linejoin":"round",d:"M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"})})})]}),(0,s.jsxs)("pre",{style:{display:"block",overflowX:"auto",background:"#002451",color:"white",padding:"1rem 12px 1rem",borderTopLeftRadius:0,borderTopRightRadius:0,borderBottomRightRadius:5,borderBottomLeftRadius:5},children:[(0,s.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,s.jsxs)("span",{children:["$ ",e]})}),this.props.result&&(0,s.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,s.jsx)("span",{children:this.props.result})})]})]})})}}},8453:(t,e,n)=>{n.d(e,{R:()=>r,x:()=>c});var o=n(6540);const s={},i=o.createContext(s);function r(t){const e=o.useContext(i);return o.useMemo((function(){return"function"==typeof t?t(e):{...e,...t}}),[e,t])}function c(t){let e;return e=t.disableParentContext?"function"==typeof t.components?t.components(s):t.components||s:r(t.components),o.createElement(i.Provider,{value:e},t.children)}}}]);