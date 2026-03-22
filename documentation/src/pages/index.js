import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

const stats = [
  { number: '30+', label: 'CLI Commands' },
  { number: '100%', label: 'Rust-Powered' },
  { number: '0', label: 'Runtime Dependencies' },
  { number: '< 10MB', label: 'Binary Size' },
];

const features = [
  {
    icon: '\u{1F680}',
    title: 'Blazing Fast',
    description: 'Written in Rust for maximum performance. Manage hundreds of resources across your clusters in seconds, not minutes.',
  },
  {
    icon: '\u{1F4E6}',
    title: 'Single Binary',
    description: 'No runtime dependencies, no virtual environment, no package manager. Just one binary that works everywhere.',
  },
  {
    icon: '\u{2601}\uFE0F',
    title: 'Cloud APIM Ready',
    description: 'Deeply integrated with Cloud APIM. Log in once and all your managed Otoroshi clusters are instantly available.',
  },
  {
    icon: '\u{1F512}',
    title: 'Secure by Default',
    description: 'TLS everywhere, secure credential storage, and support for client certificates. Your cluster connections are always protected.',
  },
  {
    icon: '\u{1F504}',
    title: 'Import & Export',
    description: 'Full import/export capabilities for your Otoroshi configurations. Backup, restore, and migrate between clusters effortlessly.',
  },
  {
    icon: '\u{1F6E0}\uFE0F',
    title: 'Resource Management',
    description: 'Create, read, update, and delete any Otoroshi resource. Routes, services, API keys, certificates — all from your terminal.',
  },
];

const capabilities = [
  {
    icon: '\u{1F3AF}',
    title: 'Multi-Cluster',
    description: 'Connect and switch between multiple Otoroshi clusters seamlessly. One tool to manage them all.',
  },
  {
    icon: '\u{1F310}',
    title: 'TCP Tunnels',
    description: 'Establish secure TCP and UDP tunnels through your Otoroshi clusters directly from the command line.',
  },
  {
    icon: '\u{1F4CB}',
    title: 'Rich Output',
    description: 'Beautiful table formatting, JSON output, and customizable display options for every command.',
  },
  {
    icon: '\u{26A1}',
    title: 'Shell Completions',
    description: 'Auto-completion for Bash, Zsh, Fish, and PowerShell. Never mistype a command again.',
  },
];

const useCases = [
  {
    icon: '\u{1F468}\u200D\u{1F4BB}',
    title: 'DevOps Workflows',
    description: 'Automate Otoroshi configuration in your CI/CD pipelines. Script everything, version control your infrastructure.',
  },
  {
    icon: '\u{1F3E2}',
    title: 'Enterprise Management',
    description: 'Manage large-scale Otoroshi deployments across multiple environments with a single, consistent interface.',
  },
  {
    icon: '\u{1F50D}',
    title: 'Debugging & Inspection',
    description: 'Quickly inspect routes, services, and configurations. Troubleshoot issues faster than navigating a web UI.',
  },
  {
    icon: '\u{1F4BE}',
    title: 'Backup & Migration',
    description: 'Export full cluster configurations, transfer between environments, and maintain disaster recovery snapshots.',
  },
];

function HomepageHeader() {
  return (
    <header className={styles.heroBanner}>
      <div className="container">
        <div className={styles.heroLayout}>
          <div className={styles.heroContent}>
            <div className={styles.heroTagline}>Command Line Power</div>
            <Heading as="h1" className={styles.heroTitle}>
              Manage Your <span className={styles.heroTitleAccent}>Otoroshi Clusters</span> with Style
            </Heading>
            <p className={styles.heroSubtitle}>
              A fast, modern CLI tool built in Rust to manage your Otoroshi clusters.
              Multi-cluster support, secure tunnels, full resource management —
              all from your terminal.
            </p>
            <div className={styles.heroButtons}>
              <Link className={styles.heroPrimary} to="/docs/overview">
                Get Started
              </Link>
              <Link
                className={styles.heroSecondary}
                href="https://github.com/cloud-apim/otoroshictl/releases/latest">
                Download
              </Link>
              <Link
                className={styles.heroGithub}
                href="https://github.com/cloud-apim/otoroshictl">
                GitHub
              </Link>
            </div>
          </div>
          <div className={styles.heroMascotWrapper}>
            <img
              src={require('@site/static/img/otoroshictl-logo-big-nobg.png').default}
              alt="otoroshictl"
              className={styles.heroMascot}
            />
          </div>
        </div>
      </div>
    </header>
  );
}

function StatsStrip() {
  return (
    <section className={styles.statsStrip}>
      <div className="container">
        <div className={styles.statsGrid}>
          {stats.map((stat, idx) => (
            <div key={idx} className={styles.statItem}>
              <span className={styles.statNumber}>{stat.number}</span>
              <span className={styles.statLabel}>{stat.label}</span>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function FeaturesSection() {
  return (
    <section className={styles.featuresSection}>
      <div className="container">
        <div className={styles.sectionHeader}>
          <div className={styles.sectionTag}>Features</div>
          <Heading as="h2" className={styles.sectionTitle}>
            Everything You Need to Manage Otoroshi
          </Heading>
          <p className={styles.sectionSubtitle}>
            A complete toolkit designed for speed, security, and seamless
            integration with your Otoroshi infrastructure.
          </p>
        </div>
        <div className={styles.featuresGrid}>
          {features.map((feature, idx) => (
            <div key={idx} className={styles.featureCard}>
              <span className={styles.featureIcon}>{feature.icon}</span>
              <div className={styles.featureTitle}>{feature.title}</div>
              <p className={styles.featureDesc}>{feature.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function CapabilitiesSection() {
  return (
    <section className={styles.capabilitiesSection}>
      <div className="container">
        <div className={styles.sectionHeader}>
          <div className={styles.sectionTag}>Capabilities</div>
          <Heading as="h2" className={styles.sectionTitle}>
            Built for Power Users
          </Heading>
          <p className={styles.sectionSubtitle}>
            Advanced features that make otoroshictl the go-to tool
            for managing Otoroshi at scale.
          </p>
        </div>
        <div className={styles.capabilitiesGrid}>
          {capabilities.map((cap, idx) => (
            <div key={idx} className={styles.capabilityCard}>
              <span className={styles.capabilityIcon}>{cap.icon}</span>
              <div className={styles.capabilityTitle}>{cap.title}</div>
              <p className={styles.capabilityDesc}>{cap.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function TerminalPreview() {
  return (
    <section className={styles.terminalSection}>
      <div className="container">
        <div className={styles.sectionHeader}>
          <div className={styles.sectionTag}>Quick Start</div>
          <Heading as="h2" className={styles.sectionTitle}>
            Up and Running in Seconds
          </Heading>
          <p className={styles.sectionSubtitle}>
            Install otoroshictl, connect to your cluster, and start managing resources immediately.
          </p>
        </div>
        <div className={styles.terminalWindow}>
          <div className={styles.terminalHeader}>
            <span className={styles.terminalDot} style={{ background: '#ff5f57' }} />
            <span className={styles.terminalDot} style={{ background: '#febc2e' }} />
            <span className={styles.terminalDot} style={{ background: '#28c840' }} />
            <span className={styles.terminalTitle}>terminal</span>
          </div>
          <div className={styles.terminalBody}>
            <div className={styles.terminalLine}>
              <span className={styles.terminalPrompt}>$</span>
              <span className={styles.terminalCommand}> otoroshictl config add-cluster --name prod --hostname otoroshi.example.com</span>
            </div>
            <div className={styles.terminalOutput}>cluster 'prod' added successfully</div>
            <div className={styles.terminalLine}>
              <span className={styles.terminalPrompt}>$</span>
              <span className={styles.terminalCommand}> otoroshictl get routes</span>
            </div>
            <div className={styles.terminalOutput}>
{`NAME                  DOMAIN                STATUS    PLUGINS
my-api                api.example.com       enabled   3
admin-dashboard       admin.example.com     enabled   5
websocket-service     ws.example.com        enabled   2`}
            </div>
            <div className={styles.terminalLine}>
              <span className={styles.terminalPrompt}>$</span>
              <span className={styles.terminalCommand}> otoroshictl tunnel tcp --remote prod-db:5432 --local 15432</span>
            </div>
            <div className={styles.terminalOutput}>tunnel open: localhost:15432 -{'>'} prod-db:5432</div>
          </div>
        </div>
      </div>
    </section>
  );
}

function UseCasesSection() {
  return (
    <section className={styles.useCasesSection}>
      <div className="container">
        <div className={styles.sectionHeader}>
          <div className={styles.sectionTag}>Use Cases</div>
          <Heading as="h2" className={styles.sectionTitle}>
            Built for Real-World Scenarios
          </Heading>
          <p className={styles.sectionSubtitle}>
            From local development to enterprise-scale operations.
          </p>
        </div>
        <div className={styles.useCasesGrid}>
          {useCases.map((uc, idx) => (
            <div key={idx} className={styles.useCaseCard}>
              <span className={styles.useCaseIcon}>{uc.icon}</span>
              <div className={styles.useCaseTitle}>{uc.title}</div>
              <p className={styles.useCaseDesc}>{uc.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function CtaSection() {
  return (
    <section className={styles.ctaSection}>
      <div className="container">
        <div className={styles.ctaBox}>
          <div className={styles.ctaContent}>
            <Heading as="h2" className={styles.ctaTitle}>
              Ready to Take Control of Your Clusters?
            </Heading>
            <p className={styles.ctaSubtitle}>
              Get started in seconds. Open source, free forever.
            </p>
            <div className={styles.ctaButtons}>
              <Link className={styles.heroPrimary} to="/docs/overview">
                Read the Docs
              </Link>
              <Link
                className={styles.heroSecondary}
                href="https://discord.gg/YRc8WEQU3E">
                Join the Community
              </Link>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

export default function Home() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title="Manage your Otoroshi clusters with style"
      description="A fast, modern CLI tool built in Rust to manage your Otoroshi clusters. Multi-cluster support, secure tunnels, full resource management.">
      <HomepageHeader />
      <main>
        <StatsStrip />
        <FeaturesSection />
        <TerminalPreview />
        <CapabilitiesSection />
        <UseCasesSection />
        <CtaSection />
      </main>
    </Layout>
  );
}
