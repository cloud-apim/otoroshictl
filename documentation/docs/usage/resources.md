---
sidebar_position: 1
---

# Otoroshi cluster resources

the main goal of `otoroshictl` is to help you manage your otoroshi cluster and its entities

but first we need to know what entities are manageable

## List managed entities

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

## Entities CRUD

you can list any kind of entity

```bash
$ otoroshictl resources get certificates

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

get one 

```bash
$ otoroshictl resources get certificates otoroshi-client

+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
| id              | name                                | description                                  | enabled | tags | metadata |
+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
| otoroshi-client | Otoroshi Default Client Certificate | Otoroshi client certificate (auto-generated) |         |  0   |    2     |
+-----------------+-------------------------------------+----------------------------------------------+---------+------+----------+
```

(don't forget about the `-o json` or `-o yaml` flag for something more detailed)

delete it

```bash
$ otoroshictl resources delete certificates otoroshi-client
```

edit it

```bash
$ otoroshictl resources edit certificates otoroshi-client
```

(it will open your current EDITOR)

and you can of course create entities like

```bash
$ otoroshictl resources create route \
  --data name=demootoroshictl \
  --data frontend.domain='demootoroshictl.oto.tools' \
  --data backend.target_url='https://mirror.otoroshi.io' \
  --data plugins.0.plugin='cp:otoroshi.next.plugins.ApikeyCalls' \
  --data plugins.1.plugin='cp:otoroshi.next.plugins.OverrideHost'
```

(this syntax is also available on the `edit`, and `patch` commands)

## Entities import

## Entities export

## Entities sync

## Entity templates

you can generate at any moment a template for any kind of entity

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

## Kubernetes commands

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