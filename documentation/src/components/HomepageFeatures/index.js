import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Easy to Use',
    Svg: require('@site/static/img/undraw_proud_coder_re_exuy.svg').default,
    description: (
      <>
        otoroshictl was designed from the ground up to be easily installed and
        used to manage your Otoroshi clusters quickly.
      </>
    ),
  },
  {
    title: 'Focus on What Matters',
    Svg: require('@site/static/img/undraw_hacker_mind_-6-y85.svg').default,
    description: (
      <>
        otoroshictl let you focus on managing your Otoroshi cluster, and we&apos;ll do all the heavy lifting for you.
      </>
    ),
  },
  {
    title: 'Cloud APIM integration',
    Svg: require('@site/static/img/cloud-apim-logo.svg').default,
    description: (
      <>
        otoroshictl is deeply integrated with Cloud APIM managed Otoroshi clusters. You just have to log in and all your clusters will be there, waiting for you !
      </>
    ),
  },
  {
    title: 'Powered by Rust',
    Svg: require('@site/static/img/undraw_pair_programming_re_or4x.svg').default,
    description: (
      <>
        otoroshictl is written in Rust, making it really fast and memory efficient so your journey with otoroshictl feels like a breeze
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row" style={{ justifyContent: 'center' }}>
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
