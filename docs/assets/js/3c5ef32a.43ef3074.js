"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[885],{1409:(o,t,e)=>{e.r(t),e.d(t,{assets:()=>a,contentTitle:()=>r,default:()=>h,frontMatter:()=>c,metadata:()=>l,toc:()=>u});var n=e(4848),s=e(8453),i=e(9229);const c={sidebar_position:1},r="Otoroshi deployments",l={id:"cloudapim/deployments",title:"Otoroshi deployments",description:"you can link your Cloud APIM account into your otoroshictl config, just do",source:"@site/docs/cloudapim/deployments.mdx",sourceDirName:"cloudapim",slug:"/cloudapim/deployments",permalink:"/otoroshictl/docs/cloudapim/deployments",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",previous:{title:"Cloud APIM integration",permalink:"/otoroshictl/docs/category/cloud-apim-integration"},next:{title:"Serverless projects",permalink:"/otoroshictl/docs/cloudapim/serverless"}},a={},u=[{value:"Cloud APIM subcommands",id:"cloud-apim-subcommands",level:2}];function d(o){const t={a:"a",code:"code",h1:"h1",h2:"h2",p:"p",...(0,s.R)(),...o.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(t.h1,{id:"otoroshi-deployments",children:"Otoroshi deployments"}),"\n",(0,n.jsxs)(t.p,{children:["you can link your ",(0,n.jsx)(t.a,{href:"https://www.cloud-apim.com",children:"Cloud APIM"})," account into your ",(0,n.jsx)(t.code,{children:"otoroshictl"})," config, just do"]}),"\n",(0,n.jsx)(i.A,{command:"otoroshictl cloud-apim login"}),"\n",(0,n.jsxs)(t.p,{children:["it should open a web browser where you can log into your ",(0,n.jsx)(t.a,{href:"https://www.cloud-apim.com",children:"Cloud APIM"})," account. Once logged in, you be able to list your deployments"]}),"\n",(0,n.jsx)(i.A,{command:"otoroshictl cloud-apim list",result:"\n\n+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+\n| name                                    | kind     | version  | status  | region | plan               | created_at               |\n+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+\n| wasi-wasm-demo                          | Otoroshi | v16.16.1 | Running | par    | xxxxxxxxxxxxxxxxxx | 2023-11-13T13:55:49.741Z |\n+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+\n| balanced analyzing budgetary management | Otoroshi | v16.16.1 | Running | par    | xxxxxxxxxxxxxxxxxx | 2024-03-07T09:55:59.424Z |\n+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+\n"}),"\n",(0,n.jsxs)(t.p,{children:["and then link a deployement to your ",(0,n.jsx)(t.code,{children:"otoroshictl"})," config"]}),"\n",(0,n.jsx)(i.A,{command:"otoroshictl cloud-apim link wasi-wasm-demo"}),"\n",(0,n.jsx)(t.h2,{id:"cloud-apim-subcommands",children:"Cloud APIM subcommands"}),"\n",(0,n.jsx)(i.A,{command:"otoroshictl cloud-apim -ht",result:"\n\nManage cloud apim clusters\n\nUsage: otoroshictl cloud-apim [OPTIONS] <COMMAND>\n\nCommands:\n  login    Login to your cloud-apim account\n  list     List your deployments\n  logout   Logout from your cloud-apim account\n  link     Add the cluster to the possible otoroshictl configs\n  use      Add the cluster to the possible otoroshictl configs and set it as the current one\n  restart  Restart this otoroshi cluster on cloud-apim\n  help     Print this message or the help of the given subcommand(s)\n\nOptions:\n  -v, --verbose\n          Turn debugging information on\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -c, --config-file <FILE or URL>\n          Sets a custom config file\n      --otoroshi-cluster-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-tls\n          Sets the tls flag to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-hostname <HOSTNAME>\n          Sets the hostname to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-routing-port <PORT>\n          Sets the port to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-id <CLIENT_ID>\n          Sets the client_id to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-client-secret <CLIENT_SECRET>\n          Sets the client_secret to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-user-health-key <HEALTH_KEY>\n          Sets the health_key to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-cert-location <FILE>\n          Sets the client cert location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-key-location <FILE>\n          Sets the client cert key location to connect to a custom otoroshi cluster without using a config file\n      --otoroshi-cluster-ca-location <FILE>\n          Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file\n  -h, --help\n          Print help\n"})]})}function h(o={}){const{wrapper:t}={...(0,s.R)(),...o.components};return t?(0,n.jsx)(t,{...o,children:(0,n.jsx)(d,{...o})}):d(o)}},9229:(o,t,e)=>{e.d(t,{A:()=>i});var n=e(6540),s=e(4848);class i extends n.Component{state={copy:!1};copy=()=>{this.setState({copy:!0},(()=>{navigator.clipboard.writeText(this.props.command),setTimeout((()=>{this.setState({copy:!1})}),1e3)}))};render(){const o=this.state.copy?"rgb(40, 167, 69)":"white";let t=(this.props.command||"").trim();return t.startsWith("\n")&&(t=t.substring(1)),t.startsWith("$ ")&&(t=t.substring(2)),(0,s.jsx)("div",{className:"terminal-component",style:{width:"100%",marginTop:20},children:(0,s.jsxs)("div",{style:{maxWidth:"100%",display:"flex",flexDirection:"column",borderRadius:5},children:[(0,s.jsxs)("div",{style:{borderTopLeftRadius:5,borderTopRightRadius:5,height:"2rem",gap:".35rem",display:"flex",flexDirection:"row",justifyContent:"center",alignItems:"center",background:"#3f52e3",color:"white"},children:[(0,s.jsx)("div",{style:{marginLeft:".35rem",width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,s.jsx)("span",{style:{width:"100%"}}),(0,s.jsx)("div",{style:{cursor:"pointer",width:30,height:30},onClick:this.copy,children:(0,s.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",fill:"none",viewBox:"0 0 24 24","stroke-width":"1.5",stroke:o,className:"w-5 h-5",children:(0,s.jsx)("path",{"stroke-linecap":"round","stroke-linejoin":"round",d:"M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"})})})]}),(0,s.jsxs)("pre",{style:{display:"block",overflowX:"auto",background:"#002451",color:"white",padding:"1rem 12px 1rem",borderTopLeftRadius:0,borderTopRightRadius:0,borderBottomRightRadius:5,borderBottomLeftRadius:5},children:[(0,s.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,s.jsxs)("span",{children:["$ ",t]})}),this.props.result&&(0,s.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,s.jsx)("span",{children:this.props.result})})]})]})})}}},8453:(o,t,e)=>{e.d(t,{R:()=>c,x:()=>r});var n=e(6540);const s={},i=n.createContext(s);function c(o){const t=n.useContext(i);return n.useMemo((function(){return"function"==typeof o?o(t):{...t,...o}}),[t,o])}function r(o){let t;return t=o.disableParentContext?"function"==typeof o.components?o.components(s):o.components||s:c(o.components),n.createElement(i.Provider,{value:t},o.children)}}}]);