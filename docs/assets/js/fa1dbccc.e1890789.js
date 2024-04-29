"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[584],{3675:(e,o,t)=>{t.r(o),t.d(o,{assets:()=>i,contentTitle:()=>r,default:()=>d,frontMatter:()=>s,metadata:()=>a,toc:()=>c});var n=t(4848),l=t(8453);const s={sidebar_position:6},r="Remote tunnels client",a={id:"remotetunnels",title:"Remote tunnels client",description:"otoroshictl is capable of talking with the otoroshi remote tunnel protocol as described here in the official otoroshi documentation.",source:"@site/docs/remotetunnels.md",sourceDirName:".",slug:"/remotetunnels",permalink:"/otoroshictl/docs/remotetunnels",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:6,frontMatter:{sidebar_position:6},sidebar:"tutorialSidebar",previous:{title:"Outside of a kubernetes cluster",permalink:"/otoroshictl/docs/sidecar/outside"},next:{title:"Cloud APIM integration",permalink:"/otoroshictl/docs/category/cloud-apim-integration"}},i={},c=[{value:"Make a local service available through remote tunnel",id:"make-a-local-service-available-through-remote-tunnel",level:2},{value:"Expose a local service available through remote tunnel",id:"expose-a-local-service-available-through-remote-tunnel",level:2},{value:"Command usage",id:"command-usage",level:2}];function h(e){const o={a:"a",code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,l.R)(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(o.h1,{id:"remote-tunnels-client",children:"Remote tunnels client"}),"\n",(0,n.jsxs)(o.p,{children:[(0,n.jsx)(o.code,{children:"otoroshictl"})," is capable of talking with the otoroshi remote tunnel protocol as described ",(0,n.jsx)(o.a,{href:"https://maif.github.io/otoroshi/manual/topics/tunnels.html",children:"here in the official otoroshi documentation"}),"."]}),"\n",(0,n.jsxs)(o.p,{children:["The idea here is to create a bidirectionnal tunnel between ",(0,n.jsx)(o.code,{children:"otoroshictl"})," and an otoroshi instance in order to make this otoroshi instance capable of exposing service only accessible to ",(0,n.jsx)(o.code,{children:"otoroshictl"}),"."]}),"\n",(0,n.jsx)(o.h2,{id:"make-a-local-service-available-through-remote-tunnel",children:"Make a local service available through remote tunnel"}),"\n",(0,n.jsxs)(o.p,{children:["here we are going to make the process listening on port 3000 on localhost available to the current otoroshi cluster. This process will be available through tunnel ",(0,n.jsx)(o.code,{children:"process1"})]}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl remote-tunnel --local-port 3000 --tunnel process1\n"})}),"\n",(0,n.jsx)(o.p,{children:"we can also make distance services available as well"}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl remote-tunnel --local-port 3000 --local-host 192.168.1.42 --tunnel process1\n"})}),"\n",(0,n.jsx)(o.h2,{id:"expose-a-local-service-available-through-remote-tunnel",children:"Expose a local service available through remote tunnel"}),"\n",(0,n.jsxs)(o.p,{children:["here we are going to make the process listening on port 3000 on localhost available to the current otoroshi cluster and automatically expose it through a route. This process will be available through tunnel ",(0,n.jsx)(o.code,{children:"process1"}),"."]}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl remote-tunnel --local-port 3000 --tunnel process1 --expose\n\n[INFO  otoroshictl::tunnels::remote]\n[INFO  otoroshictl::tunnels::remote] your service will be available at: http://967cdd29-ddd9-4d0a-a894-3b24e50f64c7-tunnel.oto.tools:8080\n[INFO  otoroshictl::tunnels::remote]\n[INFO  otoroshictl::tunnels::remote] connecting the tunnel ...\n[INFO  otoroshictl::tunnels::remote] connection done !\n"})}),"\n",(0,n.jsx)(o.p,{children:"we can also explicitely pass the exposed domain with"}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl remote-tunnel --local-port 3000 --tunnel process1 --expose --remote-domain foo.bar --remote-subdomain process1\n"})}),"\n",(0,n.jsxs)(o.p,{children:["and you'll be able to access your process at ",(0,n.jsx)(o.code,{children:"http://process1.foo.bar"})]}),"\n",(0,n.jsx)(o.h2,{id:"command-usage",children:"Command usage"}),"\n",(0,n.jsx)(o.pre,{children:(0,n.jsx)(o.code,{className:"language-bash",children:"$ otoroshictl remote-tunnel -h\n\nExposes local processes on the current otoroshi cluster through the otoroshi remote tunnel feature\n\nUsage: otoroshictl remote-tunnel [OPTIONS]\n\nOptions:\n      --local-host <LOCAL_HOST>\n          the local host forwarded to [default: localhost]\n      --local-port <LOCAL_PORT>\n          the local port forwarded to [default: 8080]\n      --local-tls\n          local process exposed as tls ?\n      --expose\n          enable expose mode\n      --remote-domain <REMOTE_DOMAIN>\n          the exposed domain\n      --remote-subdomain <REMOTE_SUBDOMAIN>\n          the exposed subdomain\n      --tls\n          enable tls want mode\n      --tunnel <TUNNEL>\n          the tunnel id [default: cli]\n  ...\n"})})]})}function d(e={}){const{wrapper:o}={...(0,l.R)(),...e.components};return o?(0,n.jsx)(o,{...e,children:(0,n.jsx)(h,{...e})}):h(e)}},8453:(e,o,t)=>{t.d(o,{R:()=>r,x:()=>a});var n=t(6540);const l={},s=n.createContext(l);function r(e){const o=n.useContext(s);return n.useMemo((function(){return"function"==typeof e?e(o):{...o,...e}}),[o,e])}function a(e){let o;return o=e.disableParentContext?"function"==typeof e.components?e.components(l):e.components||l:r(e.components),n.createElement(s.Provider,{value:o},e.children)}}}]);