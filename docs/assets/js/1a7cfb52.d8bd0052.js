"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[818],{5774:(t,o,e)=>{e.r(o),e.d(o,{assets:()=>l,contentTitle:()=>c,default:()=>d,frontMatter:()=>r,metadata:()=>a,toc:()=>u});var s=e(4848),n=e(8453),i=e(9229);const r={sidebar_position:5},c="Otoroshi cluster version",a={id:"usage/version",title:"Otoroshi cluster version",description:"at any moment you can get your cluster version with the command",source:"@site/docs/usage/version.mdx",sourceDirName:"usage",slug:"/usage/version",permalink:"/otoroshictl/docs/usage/version",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:5,frontMatter:{sidebar_position:5},sidebar:"tutorialSidebar",previous:{title:"Otoroshi cluster metrics",permalink:"/otoroshictl/docs/usage/metrics"},next:{title:"Universal mesh",permalink:"/otoroshictl/docs/category/universal-mesh"}},l={},u=[{value:"Version command usage",id:"version-command-usage",level:2}];function h(t){const o={h1:"h1",h2:"h2",p:"p",...(0,n.R)(),...t.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(o.h1,{id:"otoroshi-cluster-version",children:"Otoroshi cluster version"}),"\n",(0,s.jsx)(o.p,{children:"at any moment you can get your cluster version with the command"}),"\n",(0,s.jsx)(i.A,{command:"otoroshictl version",result:"\n\n+-------------+-------+-------+-------+-------+--------+----------------+\n| version     | major | minor | patch | build | suffix | suffix version |\n+-------------+-------+-------+-------+-------+--------+----------------+\n| 16.17.0-dev | 16    | 17    | 0     |       | dev    |                |\n+-------------+-------+-------+-------+-------+--------+----------------+\n"}),"\n",(0,s.jsx)(o.h2,{id:"version-command-usage",children:"Version command usage"}),"\n",(0,s.jsx)(i.A,{command:"otoroshictl version",result:"\n\nDisplay the version of the current otoroshi cluster\n\nUsage: otoroshictl version [OPTIONS]\n\nOptions:\n  -v, --verbose\n          Turn debugging information on\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -c, --config-file <FILE or URL>\n          Sets a custom config file\n      --otoroshi-cluster-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-id <CLIENT_ID>\n          Sets the client_id to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-secret <CLIENT_SECRET>\n          Sets the client_secret to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-health-key <HEALTH_KEY>\n          Sets the health_key to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-cert-location <FILE>\n          Sets the client cert location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-key-location <FILE>\n          Sets the client cert key location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-ca-location <FILE>\n          Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file\n  -h, --help\n          Print help\n"})]})}function d(t={}){const{wrapper:o}={...(0,n.R)(),...t.components};return o?(0,s.jsx)(o,{...t,children:(0,s.jsx)(h,{...t})}):h(t)}},9229:(t,o,e)=>{e.d(o,{A:()=>i});var s=e(6540),n=e(4848);class i extends s.Component{state={copy:!1};copy=()=>{this.setState({copy:!0},(()=>{navigator.clipboard.writeText(this.props.command),setTimeout((()=>{this.setState({copy:!1})}),1e3)}))};render(){const t=this.state.copy?"rgb(40, 167, 69)":"white";let o=(this.props.command||"").trim();return o.startsWith("\n")&&(o=o.substring(1)),o.startsWith("$ ")&&(o=o.substring(2)),(0,n.jsx)("div",{className:"terminal-component",style:{width:"100%",marginTop:20},children:(0,n.jsxs)("div",{style:{maxWidth:"100%",display:"flex",flexDirection:"column",borderRadius:5},children:[(0,n.jsxs)("div",{style:{borderTopLeftRadius:5,borderTopRightRadius:5,height:"2rem",gap:".35rem",display:"flex",flexDirection:"row",justifyContent:"center",alignItems:"center",background:"#3f52e3",color:"white"},children:[(0,n.jsx)("div",{style:{marginLeft:".35rem",width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,n.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,n.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,n.jsx)("span",{style:{width:"100%"}}),(0,n.jsx)("div",{style:{cursor:"pointer",width:30,height:30},onClick:this.copy,children:(0,n.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",fill:"none",viewBox:"0 0 24 24","stroke-width":"1.5",stroke:t,className:"w-5 h-5",children:(0,n.jsx)("path",{"stroke-linecap":"round","stroke-linejoin":"round",d:"M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"})})})]}),(0,n.jsxs)("pre",{style:{display:"block",overflowX:"auto",background:"#002451",color:"white",padding:"1rem 12px 1rem",borderTopLeftRadius:0,borderTopRightRadius:0,borderBottomRightRadius:5,borderBottomLeftRadius:5},children:[(0,n.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,n.jsxs)("span",{children:["$ ",o]})}),this.props.result&&(0,n.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,n.jsx)("span",{children:this.props.result})})]})]})})}}},8453:(t,o,e)=>{e.d(o,{R:()=>r,x:()=>c});var s=e(6540);const n={},i=s.createContext(n);function r(t){const o=s.useContext(i);return s.useMemo((function(){return"function"==typeof t?t(o):{...o,...t}}),[o,t])}function c(t){let o;return o=t.disableParentContext?"function"==typeof t.components?t.components(n):t.components||n:r(t.components),s.createElement(i.Provider,{value:o},t.children)}}}]);