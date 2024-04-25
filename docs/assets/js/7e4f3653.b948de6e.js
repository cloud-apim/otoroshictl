"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[753],{9770:(t,o,e)=>{e.r(o),e.d(o,{assets:()=>h,contentTitle:()=>i,default:()=>u,frontMatter:()=>c,metadata:()=>r,toc:()=>a});var n=e(4848),s=e(8453);const c={sidebar_position:2},i="Otoroshi cluster health",r={id:"usage/health",title:"Otoroshi cluster health",description:"at any moment you can get your cluster health with the command",source:"@site/docs/usage/health.md",sourceDirName:"usage",slug:"/usage/health",permalink:"/otoroshictl/docs/usage/health",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"tutorialSidebar",previous:{title:"Otoroshi cluster resources",permalink:"/otoroshictl/docs/usage/resources"},next:{title:"Otoroshi cluster informations",permalink:"/otoroshictl/docs/usage/infos"}},h={},a=[{value:"Health command usage",id:"health-command-usage",level:2}];function l(t){const o={code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,s.R)(),...t.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(o.h1,{id:"otoroshi-cluster-health",children:"Otoroshi cluster health"}),"\n",(0,n.jsx)(o.p,{children:"at any moment you can get your cluster health with the command"}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl health\n\n+----------+-----------+---------+------------+--------------+---------+---------+\n| otoroshi | datastore | storage | eventstore | certificates | scripts | cluster |\n+----------+-----------+---------+------------+--------------+---------+---------+\n| healthy  | healthy   | healthy | unknown    | loaded       | loaded  | healthy |\n+----------+-----------+---------+------------+--------------+---------+---------+\n"})}),"\n",(0,n.jsx)(o.h2,{id:"health-command-usage",children:"Health command usage"}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl health -h\n\nDisplay the health status of the current otoroshi cluster\n\nUsage: otoroshictl health [OPTIONS]\n\nOptions:\n  -v, --verbose\n          Turn debugging information on\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -c, --config-file <FILE or URL>\n          Sets a custom config file\n      --otoroshi-cluster-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-id <CLIENT_ID>\n          Sets the client_id to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-secret <CLIENT_SECRET>\n          Sets the client_secret to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-health-key <HEALTH_KEY>\n          Sets the health_key to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-cert-location <FILE>\n          Sets the client cert location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-key-location <FILE>\n          Sets the client cert key location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-ca-location <FILE>\n          Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file\n  -h, --help\n          Print help\n"})})]})}function u(t={}){const{wrapper:o}={...(0,s.R)(),...t.components};return o?(0,n.jsx)(o,{...t,children:(0,n.jsx)(l,{...t})}):l(t)}},8453:(t,o,e)=>{e.d(o,{R:()=>i,x:()=>r});var n=e(6540);const s={},c=n.createContext(s);function i(t){const o=n.useContext(c);return n.useMemo((function(){return"function"==typeof t?t(o):{...o,...t}}),[o,t])}function r(t){let o;return o=t.disableParentContext?"function"==typeof t.components?t.components(s):t.components||s:i(t.components),n.createElement(c.Provider,{value:o},t.children)}}}]);