"use strict";(self.webpackChunkdocumentation=self.webpackChunkdocumentation||[]).push([[656],{8705:(e,o,n)=>{n.r(o),n.d(o,{assets:()=>l,contentTitle:()=>a,default:()=>h,frontMatter:()=>i,metadata:()=>c,toc:()=>d});var t=n(4848),r=n(8453),s=n(9229);const i={sidebar_position:1},a="Otoroshi cluster resources",c={id:"usage/resources",title:"Otoroshi cluster resources",description:"the main goal of otoroshictl is to help you manage your otoroshi cluster and its entities",source:"@site/docs/usage/resources.mdx",sourceDirName:"usage",slug:"/usage/resources",permalink:"/otoroshictl/docs/usage/resources",draft:!1,unlisted:!1,tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",previous:{title:"Usage",permalink:"/otoroshictl/docs/category/usage"},next:{title:"Otoroshi cluster health",permalink:"/otoroshictl/docs/usage/health"}},l={},d=[{value:"List all managed entities",id:"list-all-managed-entities",level:2},{value:"The resources commands",id:"the-resources-commands",level:2},{value:"Get all entities of a kind",id:"get-all-entities-of-a-kind",level:2},{value:"Get one entity of a kind",id:"get-one-entity-of-a-kind",level:2},{value:"Delete one entity of a kind",id:"delete-one-entity-of-a-kind",level:2},{value:"Create one entity of a kind",id:"create-one-entity-of-a-kind",level:2},{value:"Update one entity of a kind",id:"update-one-entity-of-a-kind",level:2},{value:"Patch one entity of a kind",id:"patch-one-entity-of-a-kind",level:2},{value:"Entities import",id:"entities-import",level:2},{value:"Entities export",id:"entities-export",level:2},{value:"Entities synchronize any entity",id:"entities-synchronize-any-entity",level:2},{value:"Entity templates",id:"entity-templates",level:2},{value:"Kubernetes specific commands",id:"kubernetes-specific-commands",level:2}];function u(e){const o={admonition:"admonition",code:"code",h1:"h1",h2:"h2",p:"p",...(0,r.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(o.h1,{id:"otoroshi-cluster-resources",children:"Otoroshi cluster resources"}),"\n",(0,t.jsxs)(o.p,{children:["the main goal of ",(0,t.jsx)(o.code,{children:"otoroshictl"})," is to help you manage your otoroshi cluster and its entities"]}),"\n",(0,t.jsx)(o.p,{children:"but first we need to know what entities are manageable"}),"\n",(0,t.jsx)(o.h2,{id:"list-all-managed-entities",children:"List all managed entities"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl entities",result:"\n\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| kind              | singular_name      | plural_name         | group                              | version | served | deprecated | storage |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Route             | route              | routes              | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Backend           | backend            | backends            | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| RouteComposition  | route-composition  | route-compositions  | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| ServiceDescriptor | service-descriptor | service-descriptors | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| TcpService        | tcp-service        | tcp-services        | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| ErrorTemplate     | error-templates    | error-templates     | proxy.otoroshi.io                  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Apikey            | apikey             | apikeys             | apim.otoroshi.io                   | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Certificate       | certificate        | certificates        | pki.otoroshi.io                    | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| JwtVerifier       | jwt-verifier       | jwt-verifiers       | security.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| AuthModule        | auth-module        | auth-modules        | security.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| AdminSession      | admin-session      | admin-sessions      | security.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| SimpleAdminUser   | admins             | admins              | security.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| AuthModuleUser    | auth-module-user   | auth-module-users   | security.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| ServiceGroup      | service-group      | service-groups      | organize.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Organization      | organization       | organizations       | organize.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Tenant            | tenant             | tenants             | organize.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Team              | team               | teams               | organize.otoroshi.io               | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| DateExporter      | data-exporter      | data-exporters      | events.otoroshi.io                 | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| Script            | script             | scripts             | plugins.otoroshi.io                | v1      | true   | true       | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| WasmPlugin        | wasm-plugin        | wasm-plugins        | plugins.otoroshi.io                | v1      | true   | true       | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| GlobalConfig      | global-config      | global-configs      | config.otoroshi.io                 | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| GreenScore        | green-score        | green-scores        | green-score.extensions.otoroshi.io | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n| CorazaConfig      | coraza-config      | coraza-configs      | coraza-waf.extensions.otoroshi.io  | v1      | true   | false      | true    |\n+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+\n"}),"\n",(0,t.jsxs)(o.p,{children:["now we can start working with the ",(0,t.jsx)(o.code,{children:"otoroshictl resources"})," commands"]}),"\n",(0,t.jsx)(o.h2,{id:"the-resources-commands",children:"The resources commands"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources -h",result:"\n\nManage all the resources (entities) of the current otoroshi cluster\n\nUsage: otoroshictl resources [OPTIONS] <COMMAND>\n\nCommands:\n  template  Generate a template for the current kind\n  crds      Generate crds manifest for kubernetes\n  rbac      Generate rbac manifest for kubernetes\n  get       Get otoroshi resource from current cluster\n  delete    Delete otoroshi resources\n  patch     Update otoroshi resources through json merge or json patch\n  edit      Update otoroshi resources\n  create    Create otoroshi resources\n  apply     Synchronise otoroshi resources from files or directories\n  export    Export otoroshi resources to files or directories\n  import    Import data from an export file\n  help      Print this message or the help of the given subcommand(s)\n"}),"\n",(0,t.jsx)(o.h2,{id:"get-all-entities-of-a-kind",children:"Get all entities of a kind"}),"\n",(0,t.jsx)(o.p,{children:"you can list any kind of entity, here for instance certificates"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources get certificates",result:"\n\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| id                                            | name                                         | description                                      | enabled | tags | metadata |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| otoroshi-client                               | Otoroshi Default Client Certificate          | Otoroshi client certificate (auto-generated)     |         |  0   |    2     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| otoroshi-intermediate-ca                      | Otoroshi Default Intermediate CA Certificate | Otoroshi intermediate CA (auto-generated)        |         |  0   |    2     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| otoroshi-root-ca                              | Otoroshi Default Root CA Certificate         | Otoroshi root CA (auto-generated)                |         |  0   |    2     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| otoroshi-jwt-signing                          | Otoroshi Default Jwt Signing Keypair         | Otoroshi jwt signing keypair (auto-generated)    |         |  0   |    2     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| kubernetes-mesh-cert                          | Kubernetes Mesh Certificate                  | Kubernetes Mesh Certificate (auto-generated)     |         |  0   |    2     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n| kubernetes-webhooks-cert                      | Kubernetes Webhooks Certificate              | Kubernetes Webhooks Certificate (auto-generated) |         |  0   |    3     |\n+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+\n"}),"\n",(0,t.jsx)(o.admonition,{type:"tip",children:(0,t.jsxs)(o.p,{children:["don't forget about the ",(0,t.jsx)(o.code,{children:"-o json"})," or ",(0,t.jsx)(o.code,{children:"-o yaml"})," flag for something more detailed"]})}),"\n",(0,t.jsx)(o.p,{children:"details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources get -h",result:"\n\nGet otoroshi resource from current cluster\n\nUsage: otoroshictl resources get [OPTIONS] [RESOURCE] [ID]\n\nArguments:\n  [RESOURCE]  Optional resource name to operate on\n  [ID]        Optional resource id to operate on\n\nOptions:\n      --columns <COLUMNS>\n          Optional comma separated list of columns to display\n  -v, --verbose\n          Turn debugging information on\n  -k, --kube\n          Add kube armor to resources\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --page <PAGE>\n          The viewed page\n      --page-size <PAGE_SIZE>\n          The viewed page size\n  -f, --filters <FILTERS>\n          Filter the returned elements\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"get-one-entity-of-a-kind",children:"Get one entity of a kind"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources get certificates otoroshi-client",result:"\n\n+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+\n| id              | name                                | description                                  | enabled | tags | metadata |\n+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+\n| otoroshi-client | Otoroshi Default Client Certificate | Otoroshi client certificate (auto-generated) |         |  0   |    2     |\n+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+\n"}),"\n",(0,t.jsx)(o.admonition,{type:"tip",children:(0,t.jsxs)(o.p,{children:["don't forget about the ",(0,t.jsx)(o.code,{children:"-o json"})," or ",(0,t.jsx)(o.code,{children:"-o yaml"})," flag for something more detailed"]})}),"\n",(0,t.jsx)(o.p,{children:"details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources get -h",result:"\n\nGet otoroshi resource from current cluster\n\nUsage: otoroshictl resources get [OPTIONS] [RESOURCE] [ID]\n\nArguments:\n  [RESOURCE]  Optional resource name to operate on\n  [ID]        Optional resource id to operate on\n\nOptions:\n      --columns <COLUMNS>\n          Optional comma separated list of columns to display\n  -v, --verbose\n          Turn debugging information on\n  -k, --kube\n          Add kube armor to resources\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --page <PAGE>\n          The viewed page\n      --page-size <PAGE_SIZE>\n          The viewed page size\n  -f, --filters <FILTERS>\n          Filter the returned elements\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"delete-one-entity-of-a-kind",children:"Delete one entity of a kind"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources delete certificates otoroshi-client"}),"\n",(0,t.jsx)(o.p,{children:"details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources delete -h",result:"\n\nDelete otoroshi resources\n\nUsage: otoroshictl resources delete [OPTIONS] [RESOURCE] [IDS]...\n\nArguments:\n  [RESOURCE]  Optional resource name to operate on\n  [IDS]...    the ids to delete\n\nOptions:\n  -f, --file <FILE or URL>\n          The file to delete\n  -v, --verbose\n          Turn debugging information on\n  -d, --directory <DIR>\n          The directory to delete\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -r, --recursive\n          Walk through sub directories\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"create-one-entity-of-a-kind",children:"Create one entity of a kind"}),"\n",(0,t.jsx)(o.p,{children:"you can create an entity by passing a reference to a file or a url like"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create route -f ./route.json"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create route -f https://www.foo.bar/route.json"}),"\n",(0,t.jsx)(o.p,{children:"or using stdin"}),"\n",(0,t.jsx)(s.A,{command:"cat ./route.json | otoroshi resources create route --stdin"}),"\n",(0,t.jsxs)(o.p,{children:["or using the ",(0,t.jsx)(o.code,{children:"--data"})," flag"]}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create route \\\n  --data name=demootoroshictl \\\n  --data frontend.domain='demootoroshictl.oto.tools' \\\n  --data backend.target_url='https://mirror.otoroshi.io' \\\n  --data plugins.0.plugin='cp:otoroshi.next.plugins.ApikeyCalls' \\\n  --data plugins.1.plugin='cp:otoroshi.next.plugins.OverrideHost'"}),"\n",(0,t.jsx)(o.p,{children:"or by inlining entity content"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create route '{kind':'Route', ... }'"}),"\n",(0,t.jsx)(o.p,{children:"or finaly by using you prefered editor"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create route"}),"\n",(0,t.jsxs)(o.p,{children:["that will launch the editor defined in your ",(0,t.jsx)(o.code,{children:"EDITOR"})," env. variable"]}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources create -h",result:"\n\nCreate otoroshi resources\n\nUsage: otoroshictl resources create [OPTIONS] <RESOURCE> [INPUT]\n\nArguments:\n  <RESOURCE>  The resource name to operate on\n  [INPUT]     The optional inline entity input\n\nOptions:\n  -f, --file <FILE or URL>\n          The file to sync\n  -v, --verbose\n          Turn debugging information on\n      --data <PATH=VALUE>\n          Use inline PATH=VALUE tuples as entity input\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --stdin\n          Use stdin as entity input\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"update-one-entity-of-a-kind",children:"Update one entity of a kind"}),"\n",(0,t.jsxs)(o.p,{children:["you can update an entity using the ",(0,t.jsx)(o.code,{children:"edit"})," command. You can use it in the following manners"]}),"\n",(0,t.jsx)(o.p,{children:"using a file or an url"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit route my-route -f ./my-route.json"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit route my-route -f https://www.foo.bar/my-route.json"}),"\n",(0,t.jsx)(o.p,{children:"or using stdin"}),"\n",(0,t.jsx)(s.A,{command:"cat ./my-route.json | otoroshi resources edit route my-route --stdin"}),"\n",(0,t.jsxs)(o.p,{children:["or using the ",(0,t.jsx)(o.code,{children:"--data"})," flag"]}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit route my-route \\\n  --data name=my-route \\\n  --data frontend.domain='myroute.oto.tools' \\\n  --data backend.target_url='https://mirror.otoroshi.io' \\\n  --data plugins.0.plugin='cp:otoroshi.next.plugins.ApikeyCalls' \\\n  --data plugins.1.plugin='cp:otoroshi.next.plugins.OverrideHost'"}),"\n",(0,t.jsx)(o.p,{children:"or by inlining entity content"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit route my-route '{'kind':'Route', ... }'"}),"\n",(0,t.jsx)(o.p,{children:"or finaly by using you prefered editor"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit route my-route"}),"\n",(0,t.jsxs)(o.p,{children:["that will launch the editor defined in your ",(0,t.jsx)(o.code,{children:"EDITOR"})," env. variable"]}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources edit -h",result:"\n\nUpdate otoroshi resources\n\nUsage: otoroshictl resources edit [OPTIONS] <RESOURCE> <ID> [INPUT]\n\nArguments:\n  <RESOURCE>  The resource name to operate on\n  <ID>        The resource id to operate on\n  [INPUT]     The optional inline entity input\n\nOptions:\n  -f, --file <FILE or URL>\n          The file to sync\n  -v, --verbose\n          Turn debugging information on\n      --data <PATH=VALUE>\n          Use inline PATH=VALUE tuples as entity input\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --stdin\n          Use stdin as entity input\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"patch-one-entity-of-a-kind",children:"Patch one entity of a kind"}),"\n",(0,t.jsx)(o.p,{children:"you can also update on entity using the patch command, in that case, the format of the payload is json patch"}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources patch -h",result:"\n\nUpdate otoroshi resources through json merge or json patch\n\nUsage: otoroshictl resources patch [OPTIONS] <RESOURCE> <ID> [MERGE]\n\nArguments:\n  <RESOURCE>  The resource name to operate on\n  <ID>        The resource id to operate on\n  [MERGE]     The json object to merge\n\nOptions:\n  -f, --file <FILE or URL>\n          The file containing the json object to merge\n  -v, --verbose\n          Turn debugging information on\n      --data <PATH=VALUE>\n          Use inline PATH=VALUE tuples as entity input\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --stdin\n          Use stdin as entity input\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"entities-import",children:"Entities import"}),"\n",(0,t.jsxs)(o.p,{children:["you can import the content of an entire otoroshi cluster from an export file using the ",(0,t.jsx)(o.code,{children:"import"})," command like"]}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources import -f ./export.json"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources import -f https://www.foo.bar/export.json"}),"\n",(0,t.jsx)(o.admonition,{type:"tip",children:(0,t.jsxs)(o.p,{children:["you can use the ",(0,t.jsx)(o.code,{children:"--nd-json"})," flag if the file contains an nd-json export"]})}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources import -h",result:"\n\nImport data from an export file\n\nUsage: otoroshictl resources import [OPTIONS] --file <FILE or URL>\n\nOptions:\n  -f, --file <FILE or URL>\n          The file to import\n      --nd-json\n          import from ndjson format\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"entities-export",children:"Entities export"}),"\n",(0,t.jsxs)(o.p,{children:["you can perform otoroshi exports with the ",(0,t.jsx)(o.code,{children:"export"})," command"]}),"\n",(0,t.jsx)(o.p,{children:"you can export everything in one file"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources export -f export.json"}),"\n",(0,t.jsx)(o.p,{children:"you can also specify a directory, in that case entities will be exported with one file per kind"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources export -d export",result:"\n\n$ ls -l ./export\n\ntotal 808\n-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 admin-sessions.json\n-rw-r--r--@ 1 otoroshi  otoroshi     571 26 avr 10:46 admins.json\n-rw-r--r--@ 1 otoroshi  otoroshi   10724 26 avr 10:46 apikeys.json\n-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 auth-module-users.json\n-rw-r--r--@ 1 otoroshi  otoroshi    5633 26 avr 10:46 auth-modules.json\n-rw-r--r--@ 1 otoroshi  otoroshi    1650 26 avr 10:46 backends.json\n-rw-r--r--@ 1 otoroshi  otoroshi   72071 26 avr 10:46 certificates.json\n-rw-r--r--@ 1 otoroshi  otoroshi     823 26 avr 10:46 coraza-configs.json\n-rw-r--r--@ 1 otoroshi  otoroshi    5620 26 avr 10:46 data-exporters.json\n-rw-r--r--@ 1 otoroshi  otoroshi    8473 26 avr 10:46 error-templates.json\n-rw-r--r--@ 1 otoroshi  otoroshi   10124 26 avr 10:46 global-configs.json\n-rw-r--r--@ 1 otoroshi  otoroshi    2332 26 avr 10:46 green-scores.json\n-rw-r--r--@ 1 otoroshi  otoroshi     704 26 avr 10:46 jwt-verifiers.json\n-rw-r--r--@ 1 otoroshi  otoroshi     404 26 avr 10:46 organizations.json\n-rw-r--r--@ 1 otoroshi  otoroshi  205196 26 avr 10:46 routes.json\n-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 scripts.json\n-rw-r--r--@ 1 otoroshi  otoroshi    6590 26 avr 10:46 service-descriptors.json\n-rw-r--r--@ 1 otoroshi  otoroshi     646 26 avr 10:46 service-groups.json\n-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 tcp-services.json\n-rw-r--r--@ 1 otoroshi  otoroshi     473 26 avr 10:46 teams.json\n-rw-r--r--@ 1 otoroshi  otoroshi     392 26 avr 10:46 tenants.json\n-rw-r--r--@ 1 otoroshi  otoroshi    6086 26 avr 10:46 wasm-plugins.json\n"}),"\n",(0,t.jsx)(o.p,{children:"or ask to split everything in one file per entity"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources export -d export --split-files",result:"\n\n$ ls -l ./export\n\ntotal 0\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 admins\ndrwxr-xr-x@ 11 otoroshi  otoroshi   352 26 avr 10:50 apikeys\ndrwxr-xr-x@  5 otoroshi  otoroshi   160 26 avr 10:50 auth-modules\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:48 backends\ndrwxr-xr-x@ 13 otoroshi  otoroshi   416 26 avr 10:50 certificates\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 coraza-configs\ndrwxr-xr-x@  8 otoroshi  otoroshi   256 26 avr 10:50 data-exporters\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 error-templates\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 global-configs\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 green-scores\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 jwt-verifiers\ndrwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 organizations\ndrwxr-xr-x@ 64 otoroshi  otoroshi  2048 26 avr 10:48 routes\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:48 service-descriptors\ndrwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 service-groups\ndrwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 teams\ndrwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 tenants\ndrwxr-xr-x@  6 otoroshi  otoroshi   192 26 avr 10:50 wasm-plugins\n"}),"\n",(0,t.jsx)(o.p,{children:"you can also export in yaml and with kubernetes manifest armoring"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources export -d export --split-files -o yaml --kube"}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources export -h",result:"\n\nExport otoroshi resources to files or directories\n\nUsage: otoroshictl resources export [OPTIONS]\n\nOptions:\n  -f, --file <FILE>\n  -d, --directory <DIR>\n          The directory to sync\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n      --split-files\n          Split the export into one entity per file\n      --kube\n          Split the export into one entity per file\n      --nd-json\n          Export in ndjson format\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"entities-synchronize-any-entity",children:"Entities synchronize any entity"}),"\n",(0,t.jsxs)(o.p,{children:["with the ",(0,t.jsx)(o.code,{children:"apply"})," command you will be able to synchronize your otoroshi cluster with a files containing any kind of entity."]}),"\n",(0,t.jsx)(o.p,{children:"you can use file or urls"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources apply -f entities.json"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources apply -f https://www.foo.bar/entities.json"}),"\n",(0,t.jsx)(o.p,{children:"or even a directory"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources apply -d entities --recursive"}),"\n",(0,t.jsxs)(o.p,{children:["you can also add a ",(0,t.jsx)(o.code,{children:"--watch"})," flag to keep everything in sync as you edit files"]}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources apply -d entities --recursive --watch"}),"\n",(0,t.jsx)(o.p,{children:"the details of the command"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources apply -h",result:"\n\nSynchronise otoroshi resources from files or directories\n\nUsage: otoroshictl resources apply [OPTIONS]\n\nOptions:\n  -f, --file <FILE or URL>\n          The file to sync\n  -d, --directory <DIR>\n          The directory to sync\n  -o, --ouput <FORMAT>\n          Change the rendering format (can be one of: json, yaml, json_pretty)\n  -r, --recursive\n          Walk through sub directories\n  -w, --watch\n          Keep watching file changes\n  ...\n"}),"\n",(0,t.jsx)(o.h2,{id:"entity-templates",children:"Entity templates"}),"\n",(0,t.jsx)(o.p,{children:"you can generate at any moment a template for any kind of entity supported by the current otoroshi cluster"}),"\n",(0,t.jsx)(s.A,{command:"otoroshictl resources template apikey",result:"\n\n_loc:\n  teams:\n  - default\n  tenant: default\nallowClientIdOnly: false\nauthorizations: []\nauthorizedEntities: []\nauthorizedGroup: null\nclientId: kpw38ige9gw51ju6\nclientName: client-name-apikey\nclientSecret: 1oo2a4fishjvv8a7kn9sw984r44wc3bi4txioj0en20y2hy1uj7xk7ax78sonnis\nconstrainedServicesOnly: false\ndailyQuota: 10000000\ndescription: ''\nenabled: true\nmetadata: {}\nmonthlyQuota: 10000000\nreadOnly: false\nrestrictions:\n  allowLast: true\n  allowed: []\n  enabled: false\n  forbidden: []\n  notFound: []\nrotation:\n  enabled: false\n  gracePeriod: 168\n  nextSecret: null\n  rotationEvery: 744\ntags: []\nthrottlingQuota: 10000000\nvalidUntil: null\n"}),"\n",(0,t.jsx)(o.h2,{id:"kubernetes-specific-commands",children:"Kubernetes specific commands"}),"\n",(0,t.jsxs)(o.p,{children:[(0,t.jsx)(o.code,{children:"otoroshictl"})," can help you generate your ",(0,t.jsx)(o.code,{children:"rbac"})," and ",(0,t.jsx)(o.code,{children:"crds"})," manifests"]}),"\n",(0,t.jsx)(s.A,{not_ended:!0,command:"otoroshictl resources crds",result:'\n\n---\napiVersion: "apiextensions.k8s.io/v1"\nkind: "CustomResourceDefinition"\nmetadata:\n  name: "routes.proxy.otoroshi.io"\nspec:\n  group: "proxy.otoroshi.io"\n  names:\n    kind: "Route"\n    plural: "routes"\n    singular: "route"\n  scope: "Namespaced"\n  versions:\n  - name: "v1alpha1"\n    served: false\n    storage: false\n    deprecated: true\n    schema:\n      openAPIV3Schema:\n        x-kubernetes-preserve-unknown-fields: true\n        type: "object"\n  - name: "v1"\n    served: true\n    storage: true\n    deprecated: false\n    schema:\n      openAPIV3Schema:\n        x-kubernetes-preserve-unknown-fields: true\n        type: "object"\n---\napiVersion: "apiextensions.k8s.io/v1"\nkind: "CustomResourceDefinition"\nmetadata:\n  name: "backends.proxy.otoroshi.io"\nspec:\n  group: "proxy.otoroshi.io"\n  names:\n    kind: "Backend"\n    plural: "backends"\n    singular: "backend"\n  scope: "Namespaced"\n  versions:\n  - name: "v1alpha1"\n    served: false\n    storage: false\n    deprecated: true\n    schema:\n      openAPIV3Schema:\n        x-kubernetes-preserve-unknown-fields: true\n        type: "object"\n  - name: "v1"\n    served: true\n    storage: true\n    deprecated: false\n    schema:\n      openAPIV3Schema:\n        x-kubernetes-preserve-unknown-fields: true\n        type: "object"\n...\n'}),"\n",(0,t.jsx)(s.A,{not_ended:!0,command:"otoroshictl resources rbac",result:'\n\n---\nkind: ServiceAccount\napiVersion: v1\nmetadata:\n    name: otoroshi-admin-user\n---\napiVersion: rbac.authorization.k8s.io/v1\nkind: ClusterRoleBinding\nmetadata:\n    name: otoroshi-admin-user\nroleRef:\n    apiGroup: rbac.authorization.k8s.io\n    kind: ClusterRole\n    name: otoroshi-admin-user\nsubjects:\n- kind: ServiceAccount\n    name: otoroshi-admin-user\n    namespace: $namespace\n---\nkind: ClusterRole\napiVersion: rbac.authorization.k8s.io/v1\nmetadata:\n    name: otoroshi-admin-user\nrules:\n    - apiGroups:\n        - ""\n    resources:\n        - services\n        - endpoints\n        - secrets\n        - configmaps\n        - deployments\n        - namespaces\n        - pods\n    verbs:\n        - get\n        - list\n        - watch\n    - apiGroups:\n        - "apps"\n    resources:\n        - deployments\n    verbs:\n        - get\n        - list\n        - watch\n    - apiGroups:\n        - ""\n    resources:\n        - secrets\n        - configmaps\n    verbs:\n        - update\n        - create\n        - delete\n    - apiGroups:\n        - extensions\n    resources:\n        - ingresses\n        - ingressclasses\n    verbs:\n        - get\n        - list\n        - watch\n    - apiGroups:\n        - extensions\n    resources:\n        - ingresses/status\n    verbs:\n        - update\n    - apiGroups:\n        - admissionregistration.k8s.io\n    resources:\n        - validatingwebhookconfigurations\n        - mutatingwebhookconfigurations\n    verbs:\n        - get\n        - update\n        - patch\n    - apiGroups:\n        - proxy.otoroshi.io\n    resources:\n        - routes\n        - backends\n        - route-compositions\n        - service-descriptors\n        - tcp-services\n        - error-templates\n        - apikeys\n        - certificates\n        - jwt-verifiers\n        - auth-modules\n        - admin-sessions\n        - admins\n        - auth-module-users\n        - service-groups\n        - organizations\n        - tenants\n        - teams\n        - data-exporters\n        - scripts\n        - wasm-plugins\n        - global-configs\n        - green-scores\n        - coraza-configs\n    verbs:\n        - get\n        - list\n        - watch\n'})]})}function h(e={}){const{wrapper:o}={...(0,r.R)(),...e.components};return o?(0,t.jsx)(o,{...e,children:(0,t.jsx)(u,{...e})}):u(e)}},9229:(e,o,n)=>{n.d(o,{A:()=>s});var t=n(6540),r=n(4848);class s extends t.Component{state={copy:!1};copy=()=>{this.setState({copy:!0},(()=>{navigator.clipboard.writeText(this.props.command),setTimeout((()=>{this.setState({copy:!1})}),1e3)}))};render(){const e=this.state.copy?"rgb(40, 167, 69)":"white";let o=(this.props.command||"").trim();return o.startsWith("\n")&&(o=o.substring(1)),o.startsWith("$ ")&&(o=o.substring(2)),(0,r.jsx)("div",{className:"terminal-component",style:{width:"100%",marginTop:20},children:(0,r.jsxs)("div",{style:{maxWidth:"100%",display:"flex",flexDirection:"column",borderRadius:5},children:[(0,r.jsxs)("div",{style:{borderTopLeftRadius:5,borderTopRightRadius:5,height:"2rem",gap:".35rem",display:"flex",flexDirection:"row",justifyContent:"center",alignItems:"center",background:"#3f52e3",color:"white"},children:[(0,r.jsx)("div",{style:{marginLeft:".35rem",width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,r.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,r.jsx)("div",{style:{width:10,height:10,borderRadius:"50%",backgroundColor:"#ccc"}}),(0,r.jsx)("span",{style:{width:"100%"}}),(0,r.jsx)("div",{style:{cursor:"pointer",width:30,height:30},onClick:this.copy,children:(0,r.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",fill:"none",viewBox:"0 0 24 24","stroke-width":"1.5",stroke:e,className:"w-5 h-5",children:(0,r.jsx)("path",{"stroke-linecap":"round","stroke-linejoin":"round",d:"M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"})})})]}),(0,r.jsxs)("pre",{style:{display:"block",overflowX:"auto",background:"#002451",color:"white",padding:"1rem 12px 1rem",borderTopLeftRadius:0,borderTopRightRadius:0,borderBottomRightRadius:5,borderBottomLeftRadius:5},children:[(0,r.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,r.jsxs)("span",{children:["$ ",o]})}),this.props.result&&(0,r.jsx)("code",{className:"language-bash",style:{whiteSpace:"pre"},children:(0,r.jsx)("span",{children:this.props.result})})]})]})})}}},8453:(e,o,n)=>{n.d(o,{R:()=>i,x:()=>a});var t=n(6540);const r={},s=t.createContext(r);function i(e){const o=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(o):{...o,...e}}),[o,e])}function a(e){let o;return o=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:i(e.components),t.createElement(s.Provider,{value:o},e.children)}}}]);