---
sidebar_position: 1
---

# Otoroshi cluster resources

the main goal of `otoroshictl` is to help you manage your otoroshi cluster and its entities

but first we need to know what entities are manageable

## List all managed entities

```bash
$ otoroshictl entities

+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| kind              | singular_name      | plural_name         | group                              | version | served | deprecated | storage |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Route             | route              | routes              | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Backend           | backend            | backends            | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| RouteComposition  | route-composition  | route-compositions  | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| ServiceDescriptor | service-descriptor | service-descriptors | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| TcpService        | tcp-service        | tcp-services        | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| ErrorTemplate     | error-templates    | error-templates     | proxy.otoroshi.io                  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Apikey            | apikey             | apikeys             | apim.otoroshi.io                   | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Certificate       | certificate        | certificates        | pki.otoroshi.io                    | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| JwtVerifier       | jwt-verifier       | jwt-verifiers       | security.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| AuthModule        | auth-module        | auth-modules        | security.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| AdminSession      | admin-session      | admin-sessions      | security.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| SimpleAdminUser   | admins             | admins              | security.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| AuthModuleUser    | auth-module-user   | auth-module-users   | security.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| ServiceGroup      | service-group      | service-groups      | organize.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Organization      | organization       | organizations       | organize.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Tenant            | tenant             | tenants             | organize.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Team              | team               | teams               | organize.otoroshi.io               | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| DateExporter      | data-exporter      | data-exporters      | events.otoroshi.io                 | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| Script            | script             | scripts             | plugins.otoroshi.io                | v1      | true   | true       | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| WasmPlugin        | wasm-plugin        | wasm-plugins        | plugins.otoroshi.io                | v1      | true   | true       | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| GlobalConfig      | global-config      | global-configs      | config.otoroshi.io                 | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| GreenScore        | green-score        | green-scores        | green-score.extensions.otoroshi.io | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
| CorazaConfig      | coraza-config      | coraza-configs      | coraza-waf.extensions.otoroshi.io  | v1      | true   | false      | true    |
+-------------------+--------------------+---------------------+------------------------------------+---------+--------+------------+---------+
```

now we can start working with the `otoroshictl resources` commands

## The resources commands

```bash
$ otoroshictl resources -h

Manage all the resources (entities) of the current otoroshi cluster

Usage: otoroshictl resources [OPTIONS] <COMMAND>

Commands:
  template  Generate a template for the current kind
  crds      Generate crds manifest for kubernetes
  rbac      Generate rbac manifest for kubernetes
  get       Get otoroshi resource from current cluster
  delete    Delete otoroshi resources
  patch     Update otoroshi resources through json merge or json patch
  edit      Update otoroshi resources
  create    Create otoroshi resources
  apply     Synchronise otoroshi resources from files or directories
  export    Export otoroshi resources to files or directories
  import    Import data from an export file
  help      Print this message or the help of the given subcommand(s)
```

## Get all entities of a kind

you can list any kind of entity, here for instance certificates

```bash
$ otoroshictl resources get certificates

+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| id                                            | name                                         | description                                      | enabled | tags | metadata |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| otoroshi-client                               | Otoroshi Default Client Certificate          | Otoroshi client certificate (auto-generated)     |         |  0   |    2     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| otoroshi-intermediate-ca                      | Otoroshi Default Intermediate CA Certificate | Otoroshi intermediate CA (auto-generated)        |         |  0   |    2     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| otoroshi-root-ca                              | Otoroshi Default Root CA Certificate         | Otoroshi root CA (auto-generated)                |         |  0   |    2     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| otoroshi-jwt-signing                          | Otoroshi Default Jwt Signing Keypair         | Otoroshi jwt signing keypair (auto-generated)    |         |  0   |    2     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| kubernetes-mesh-cert                          | Kubernetes Mesh Certificate                  | Kubernetes Mesh Certificate (auto-generated)     |         |  0   |    2     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
| kubernetes-webhooks-cert                      | Kubernetes Webhooks Certificate              | Kubernetes Webhooks Certificate (auto-generated) |         |  0   |    3     |
+-----------------------------------------------+----------------------------------------------+--------------------------------------------------+---------+------+----------+
```

:::tip

don't forget about the `-o json` or `-o yaml` flag for something more detailed

:::

details of the command

```bash
$ otoroshictl resources get -h

Get otoroshi resource from current cluster

Usage: otoroshictl resources get [OPTIONS] [RESOURCE] [ID]

Arguments:
  [RESOURCE]  Optional resource name to operate on
  [ID]        Optional resource id to operate on

Options:
      --columns <COLUMNS>
          Optional comma separated list of columns to display
  -v, --verbose
          Turn debugging information on
  -k, --kube
          Add kube armor to resources
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --page <PAGE>
          The viewed page
      --page-size <PAGE_SIZE>
          The viewed page size
  -f, --filters <FILTERS>
          Filter the returned elements
  ...
```

## Get one entity of a kind

```bash
$ otoroshictl resources get certificates otoroshi-client

+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
| id              | name                                | description                                  | enabled | tags | metadata |
+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
| otoroshi-client | Otoroshi Default Client Certificate | Otoroshi client certificate (auto-generated) |         |  0   |    2     |
+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
```

:::tip

don't forget about the `-o json` or `-o yaml` flag for something more detailed

:::

details of the command

```bash
$ otoroshictl resources get -h

Get otoroshi resource from current cluster

Usage: otoroshictl resources get [OPTIONS] [RESOURCE] [ID]

Arguments:
  [RESOURCE]  Optional resource name to operate on
  [ID]        Optional resource id to operate on

Options:
      --columns <COLUMNS>
          Optional comma separated list of columns to display
  -v, --verbose
          Turn debugging information on
  -k, --kube
          Add kube armor to resources
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --page <PAGE>
          The viewed page
      --page-size <PAGE_SIZE>
          The viewed page size
  -f, --filters <FILTERS>
          Filter the returned elements
  ...
```

## Delete one entity of a kind

```bash
$ otoroshictl resources delete certificates otoroshi-client
```

details of the command

```bash
$ otoroshictl resources delete -h

Delete otoroshi resources

Usage: otoroshictl resources delete [OPTIONS] [RESOURCE] [IDS]...

Arguments:
  [RESOURCE]  Optional resource name to operate on
  [IDS]...    the ids to delete

Options:
  -f, --file <FILE or URL>
          The file to delete
  -v, --verbose
          Turn debugging information on
  -d, --directory <DIR>
          The directory to delete
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
  -r, --recursive
          Walk through sub directories
  ...
```

## Create one entity of a kind

you can create an entity by passing a reference to a file or a url like

```bash
$ otoroshictl resources create route -f ./route.json
$ otoroshictl resources create route -f https://www.foo.bar/route.json
```

or using stdin

```bash
$ cat ./route.json | otoroshi resources create route --stdin
```

or using the `--data` flag

```bash
$ otoroshictl resources create route \
  --data name=demootoroshictl \
  --data frontend.domain='demootoroshictl.oto.tools' \
  --data backend.target_url='https://mirror.otoroshi.io' \
  --data plugins.0.plugin='cp:otoroshi.next.plugins.ApikeyCalls' \
  --data plugins.1.plugin='cp:otoroshi.next.plugins.OverrideHost'
```

or by inlining entity content

```bash
$ otoroshictl resources create route '{"kind":"Route", ... }'
```

or finaly by using you prefered editor 

```bash
$ otoroshictl resources create route
```

that will launch the editor defined in your `EDITOR` env. variable

the details of the command

```bash
$ otoroshictl resources create -h

Create otoroshi resources

Usage: otoroshictl resources create [OPTIONS] <RESOURCE> [INPUT]

Arguments:
  <RESOURCE>  The resource name to operate on
  [INPUT]     The optional inline entity input

Options:
  -f, --file <FILE or URL>
          The file to sync
  -v, --verbose
          Turn debugging information on
      --data <PATH=VALUE>
          Use inline PATH=VALUE tuples as entity input
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --stdin
          Use stdin as entity input
  ...
```

## Update one entity of a kind

you can update an entity using the `edit` command. You can use it in the following manners

using a file or an url

```bash
$ otoroshictl resources edit route my-route -f ./my-route.json
$ otoroshictl resources edit route my-route -f https://www.foo.bar/my-route.json
```

or using stdin

```bash
$ cat ./my-route.json | otoroshi resources edit route my-route --stdin
```

or using the `--data` flag

```bash
$ otoroshictl resources edit route my-route \
  --data name=my-route \
  --data frontend.domain='myroute.oto.tools' \
  --data backend.target_url='https://mirror.otoroshi.io' \
  --data plugins.0.plugin='cp:otoroshi.next.plugins.ApikeyCalls' \
  --data plugins.1.plugin='cp:otoroshi.next.plugins.OverrideHost'
```

or by inlining entity content

```bash
$ otoroshictl resources edit route my-route '{"kind":"Route", ... }'
```

or finaly by using you prefered editor 

```bash
$ otoroshictl resources edit route my-route
```

that will launch the editor defined in your `EDITOR` env. variable

the details of the command

```bash
$ otoroshictl resources edit -h

Update otoroshi resources

Usage: otoroshictl resources edit [OPTIONS] <RESOURCE> <ID> [INPUT]

Arguments:
  <RESOURCE>  The resource name to operate on
  <ID>        The resource id to operate on
  [INPUT]     The optional inline entity input

Options:
  -f, --file <FILE or URL>
          The file to sync
  -v, --verbose
          Turn debugging information on
      --data <PATH=VALUE>
          Use inline PATH=VALUE tuples as entity input
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --stdin
          Use stdin as entity input
  ...
```

## Patch one entity of a kind

you can also update on entity using the patch command, in that case, the format of the payload is json patch

the details of the command

```bash
$ otoroshictl resources patch -h

Update otoroshi resources through json merge or json patch

Usage: otoroshictl resources patch [OPTIONS] <RESOURCE> <ID> [MERGE]

Arguments:
  <RESOURCE>  The resource name to operate on
  <ID>        The resource id to operate on
  [MERGE]     The json object to merge

Options:
  -f, --file <FILE or URL>
          The file containing the json object to merge
  -v, --verbose
          Turn debugging information on
      --data <PATH=VALUE>
          Use inline PATH=VALUE tuples as entity input
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --stdin
          Use stdin as entity input
  ...
```

## Entities import

you can import the content of an entire otoroshi cluster from an export file using the `import` command like

```bash
$ otoroshictl resources import -f ./export.json
$ otoroshictl resources import -f https://www.foo.bar/export.json
```

:::tip

you can use the `--nd-json` flag if the file contains an nd-json export

:::

the details of the command

```bash
$ otoroshictl resources import -h

Import data from an export file

Usage: otoroshictl resources import [OPTIONS] --file <FILE or URL>

Options:
  -f, --file <FILE or URL>
          The file to import
      --nd-json
          import from ndjson format
  ...
```

## Entities export

you can perform otoroshi exports with the `export` command

you can export everything in one file

```bash
$ otoroshictl resources export -f export.json
```

you can also specify a directory, in that case entities will be exported with one file per kind

```bash
$ otoroshictl resources export -d export
$ ls -l ./export

total 808
-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 admin-sessions.json
-rw-r--r--@ 1 otoroshi  otoroshi     571 26 avr 10:46 admins.json
-rw-r--r--@ 1 otoroshi  otoroshi   10724 26 avr 10:46 apikeys.json
-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 auth-module-users.json
-rw-r--r--@ 1 otoroshi  otoroshi    5633 26 avr 10:46 auth-modules.json
-rw-r--r--@ 1 otoroshi  otoroshi    1650 26 avr 10:46 backends.json
-rw-r--r--@ 1 otoroshi  otoroshi   72071 26 avr 10:46 certificates.json
-rw-r--r--@ 1 otoroshi  otoroshi     823 26 avr 10:46 coraza-configs.json
-rw-r--r--@ 1 otoroshi  otoroshi    5620 26 avr 10:46 data-exporters.json
-rw-r--r--@ 1 otoroshi  otoroshi    8473 26 avr 10:46 error-templates.json
-rw-r--r--@ 1 otoroshi  otoroshi   10124 26 avr 10:46 global-configs.json
-rw-r--r--@ 1 otoroshi  otoroshi    2332 26 avr 10:46 green-scores.json
-rw-r--r--@ 1 otoroshi  otoroshi     704 26 avr 10:46 jwt-verifiers.json
-rw-r--r--@ 1 otoroshi  otoroshi     404 26 avr 10:46 organizations.json
-rw-r--r--@ 1 otoroshi  otoroshi  205196 26 avr 10:46 routes.json
-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 scripts.json
-rw-r--r--@ 1 otoroshi  otoroshi    6590 26 avr 10:46 service-descriptors.json
-rw-r--r--@ 1 otoroshi  otoroshi     646 26 avr 10:46 service-groups.json
-rw-r--r--@ 1 otoroshi  otoroshi       2 26 avr 10:46 tcp-services.json
-rw-r--r--@ 1 otoroshi  otoroshi     473 26 avr 10:46 teams.json
-rw-r--r--@ 1 otoroshi  otoroshi     392 26 avr 10:46 tenants.json
-rw-r--r--@ 1 otoroshi  otoroshi    6086 26 avr 10:46 wasm-plugins.json
```

or ask to split everything in one file per entity

```bash
$ otoroshictl resources export -d export --split-files
$ ls -l ./export

total 0
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 admins
drwxr-xr-x@ 11 otoroshi  otoroshi   352 26 avr 10:50 apikeys
drwxr-xr-x@  5 otoroshi  otoroshi   160 26 avr 10:50 auth-modules
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:48 backends
drwxr-xr-x@ 13 otoroshi  otoroshi   416 26 avr 10:50 certificates
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 coraza-configs
drwxr-xr-x@  8 otoroshi  otoroshi   256 26 avr 10:50 data-exporters
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 error-templates
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 global-configs
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 green-scores
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 jwt-verifiers
drwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 organizations
drwxr-xr-x@ 64 otoroshi  otoroshi  2048 26 avr 10:48 routes
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:48 service-descriptors
drwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 service-groups
drwxr-xr-x@  3 otoroshi  otoroshi    96 26 avr 10:50 teams
drwxr-xr-x@  4 otoroshi  otoroshi   128 26 avr 10:50 tenants
drwxr-xr-x@  6 otoroshi  otoroshi   192 26 avr 10:50 wasm-plugins
```

you can also export in yaml and with kubernetes manifest armoring

```bash
$ otoroshictl resources export -d export --split-files -o yaml --kube
```

the details of the command

```bash
$ otoroshictl resources export -h

Export otoroshi resources to files or directories

Usage: otoroshictl resources export [OPTIONS]

Options:
  -f, --file <FILE>
  -d, --directory <DIR>
          The directory to sync
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
      --split-files
          Split the export into one entity per file
      --kube
          Split the export into one entity per file
      --nd-json
          Export in ndjson format
  ...
```

## Entities synchronize any entity

with the `apply` command you will be able to synchronize your otoroshi cluster with a files containing any kind of entity.

you can use file or urls

```bash
$ otoroshictl resources apply -f entities.json
$ otoroshictl resources apply -f https://www.foo.bar/entities.json
```

or even a directory

```bash
$ otoroshictl resources apply -d entities --recursive
```

you can also add a `--watch` flag to keep everything in sync as you edit files

```bash
$ otoroshictl resources apply -d entities --recursive --watch
```

the details of the command

```bash
$ otoroshictl resources apply -h

Synchronise otoroshi resources from files or directories

Usage: otoroshictl resources apply [OPTIONS]

Options:
  -f, --file <FILE or URL>
          The file to sync
  -d, --directory <DIR>
          The directory to sync
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
  -r, --recursive
          Walk through sub directories
  -w, --watch
          Keep watching file changes
  ...
```

## Entity templates

you can generate at any moment a template for any kind of entity supported by the current otoroshi cluster

```bash
$ otoroshictl resources template apikey

_loc:
  teams:
  - default
  tenant: default
allowClientIdOnly: false
authorizations: []
authorizedEntities: []
authorizedGroup: null
clientId: kpw38ige9gw51ju6
clientName: client-name-apikey
clientSecret: 1oo2a4fishjvv8a7kn9sw984r44wc3bi4txioj0en20y2hy1uj7xk7ax78sonnis
constrainedServicesOnly: false
dailyQuota: 10000000
description: ''
enabled: true
metadata: {}
monthlyQuota: 10000000
readOnly: false
restrictions:
  allowLast: true
  allowed: []
  enabled: false
  forbidden: []
  notFound: []
rotation:
  enabled: false
  gracePeriod: 168
  nextSecret: null
  rotationEvery: 744
tags: []
throttlingQuota: 10000000
validUntil: null
```

## Kubernetes specific commands

`otoroshictl` can help you generate your `rbac` and `crds` manifests

```bash
$ otoroshictl resources crds

---
apiVersion: "apiextensions.k8s.io/v1"
kind: "CustomResourceDefinition"
metadata:
  name: "routes.proxy.otoroshi.io"
spec:
  group: "proxy.otoroshi.io"
  names:
    kind: "Route"
    plural: "routes"
    singular: "route"
  scope: "Namespaced"
  versions:
  - name: "v1alpha1"
    served: false
    storage: false
    deprecated: true
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: "object"
  - name: "v1"
    served: true
    storage: true
    deprecated: false
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: "object"
---
apiVersion: "apiextensions.k8s.io/v1"
kind: "CustomResourceDefinition"
metadata:
  name: "backends.proxy.otoroshi.io"
spec:
  group: "proxy.otoroshi.io"
  names:
    kind: "Backend"
    plural: "backends"
    singular: "backend"
  scope: "Namespaced"
  versions:
  - name: "v1alpha1"
    served: false
    storage: false
    deprecated: true
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: "object"
  - name: "v1"
    served: true
    storage: true
    deprecated: false
    schema:
      openAPIV3Schema:
        x-kubernetes-preserve-unknown-fields: true
        type: "object"
...
```

```bash
$ otoroshictl resources rbac

---
kind: ServiceAccount
apiVersion: v1
metadata:
    name: otoroshi-admin-user
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
    name: otoroshi-admin-user
roleRef:
    apiGroup: rbac.authorization.k8s.io
    kind: ClusterRole
    name: otoroshi-admin-user
subjects:
- kind: ServiceAccount
    name: otoroshi-admin-user
    namespace: $namespace
---
kind: ClusterRole
apiVersion: rbac.authorization.k8s.io/v1
metadata:
    name: otoroshi-admin-user
rules:
    - apiGroups:
        - ""
    resources:
        - services
        - endpoints
        - secrets
        - configmaps
        - deployments
        - namespaces
        - pods
    verbs:
        - get
        - list
        - watch
    - apiGroups:
        - "apps"
    resources:
        - deployments
    verbs:
        - get
        - list
        - watch
    - apiGroups:
        - ""
    resources:
        - secrets
        - configmaps
    verbs:
        - update
        - create
        - delete
    - apiGroups:
        - extensions
    resources:
        - ingresses
        - ingressclasses
    verbs:
        - get
        - list
        - watch
    - apiGroups:
        - extensions
    resources:
        - ingresses/status
    verbs:
        - update
    - apiGroups:
        - admissionregistration.k8s.io
    resources:
        - validatingwebhookconfigurations
        - mutatingwebhookconfigurations
    verbs:
        - get
        - update
        - patch
    - apiGroups:
        - proxy.otoroshi.io
    resources:
        - routes
        - backends
        - route-compositions
        - service-descriptors
        - tcp-services
        - error-templates
        - apikeys
        - certificates
        - jwt-verifiers
        - auth-modules
        - admin-sessions
        - admins
        - auth-module-users
        - service-groups
        - organizations
        - tenants
        - teams
        - data-exporters
        - scripts
        - wasm-plugins
        - global-configs
        - green-scores
        - coraza-configs
    verbs:
        - get
        - list
        - watch
```