---
sidebar_position: 1
---

# Setup otoroshictl

`otoroshictl` is capable of managing multiple otoroshi clusters with multiple users, but the first thing we need to make is to create the `otoroshictl` config file.

To do that, try

```bash
$ otoroshictl config reset
```

then you'll be able to list the otoroshi cluster you can manage with 

```bash
$ otoroshictl config list

+---------+---------+------------+
| name    | current | cloud_apim |
+---------+---------+------------+
| default | yes     |            |
+---------+---------+------------+
```

and display the current one

```bash
$ otoroshictl config current-config

---
apiVersion: v1
kind: OtoroshiCtlConfig
metadata: {}
cloud_apim: ~
users:
  - name: default
    client_id: admin-api-apikey-id
    client_secret: admin-api-apikey-secret
    health_key: ~
contexts:
  - name: default
    cluster: default
    user: default
    cloud_apim: false
clusters:
  - name: default
    hostname: otoroshi-api.oto.tools
    ip_addresses: ~
    port: 8080
    tls: false
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: ~
    routing_ip_addresses: ~
current_context: default
```

by default the registered otoroshi cluster is supposed to be local and use default credentials, but you can modify it of even create a new one

## Create a new cluster

to create a new cluster configuration just do the following

```bash
$ otoroshictl config add new-cluster --hostname otoroshi.foo.bar --port 8443 --tls --client-id xxx --client-secret xxxxx
```

you can even add `--current` to make it the current one

now if you list your clusters you have 

```bash
$ otoroshictl config list

+-------------+---------+------------+
| name        | current | cloud_apim |
+-------------+---------+------------+
| default     |         |            |
+-------------+---------+------------+
| new-cluster | yes     |            |
+-------------+---------+------------+
```

with the content

```bash
$ otoroshictl config current-config

---
apiVersion: v1
kind: OtoroshiCtlConfig
metadata: {}
cloud_apim: ~
users:
  - name: default
    client_id: admin-api-apikey-id
    client_secret: admin-api-apikey-secret
    health_key: ~
  - name: new-cluster
    client_id: xxx
    client_secret: xxxxx
    health_key: ~
contexts:
  - name: default
    cluster: default
    user: default
    cloud_apim: false
  - name: new-cluster
    cluster: new-cluster
    user: new-cluster
    cloud_apim: false
clusters:
  - name: default
    hostname: otoroshi-api.oto.tools
    ip_addresses: ~
    port: 8080
    tls: false
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: ~
    routing_ip_addresses: ~
  - name: new-cluster
    hostname: otoroshi.foo.bar
    ip_addresses: ~
    port: 8443
    tls: true
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: false
    routing_ip_addresses: ~
current_context: new-cluster
```

## Change the current config.

you can change the current config at any moment using the `use` command

```bash
$ otoroshictl config list

+-------------+---------+------------+
| name        | current | cloud_apim |
+-------------+---------+------------+
| default     | yes     |            |
+-------------+---------+------------+
| new-cluster |         |            |
+-------------+---------+------------+

$ otoroshictl config use new-cluster
$ otoroshictl config list

+-------------+---------+------------+
| name        | current | cloud_apim |
+-------------+---------+------------+
| default     |         |            |
+-------------+---------+------------+
| new-cluster | yes     |            |
+-------------+---------+------------+

```

## Modify an existing cluster

you can also change an existing configuration with the commands `set-cluster`, `set-user`, `set-context`

```bash
$ otoroshictl config set-cluster new-cluster --hostname otoroshi.bar.foo --port 8080 --tls false
$ otoroshictl config set-user new-cluster --client-id yyy ---client-id yyyyyyy

$ otoroshictl config current-config

---
apiVersion: v1
kind: OtoroshiCtlConfig
metadata: {}
cloud_apim: ~
users:
  - name: default
    client_id: admin-api-apikey-id
    client_secret: admin-api-apikey-secret
    health_key: ~
  - name: new-cluster
    client_id: yyy
    client_secret: yyyyyyy
    health_key: ~
contexts:
  - name: default
    cluster: default
    user: default
    cloud_apim: false
  - name: new-cluster
    cluster: new-cluster
    user: new-cluster
    cloud_apim: false
clusters:
  - name: default
    hostname: otoroshi-api.oto.tools
    ip_addresses: ~
    port: 8080
    tls: false
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: ~
    routing_ip_addresses: ~
  - name: new-cluster
    hostname: otoroshi.bar.foo
    ip_addresses: ~
    port: 8080
    tls: false
    client_cert: ~
    routing_hostname: ~
    routing_port: ~
    routing_tls: false
    routing_ip_addresses: ~
current_context: new-cluster
```

## All possible config. subcommands

```bash
$ otoroshictl config -h

Manage all the otoroshi cluster configurations you want to connect to with otoroshictl

Usage: otoroshictl config [OPTIONS] <COMMAND>

Commands:
  current-config       Display the current config. file content
  edit-current-config  Edit the current config. file
  current-location     Display current config. location
  current-context      Display current context
  use-context          Set the current context
  use                  Set the current context
  rename-context       Rename a context
  list                 Display the list of usable contexts
  list-clusters        Display the list of clusters
  list-users           Display the list of users
  list-contexts        Display the list of contexts
  set-cluster          Create or update a cluster
  set-user             Create or update a user
  set-context          Create or update a context
  add                  Create and set a full config
  delete-cluster       Delete a cluster
  delete-user          Delete a user
  delete-context       Delete a context
  delete               Delete a full context with the associated cluster and user
  reset                Delete configuration and start with a clean one
  import               Import a context file with current context file
  help                 Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Turn debugging information on
  -o, --ouput <FORMAT>
          Change the rendering format (can be one of: json, yaml, json_pretty)
  -c, --config-file <FILE or URL>
          Sets a custom config file
      --otoroshi-cluster-tls
          Sets the tls flag to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-hostname <HOSTNAME>
          Sets the hostname to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-port <PORT>
          Sets the port to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-routing-tls
          Sets the tls flag to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-routing-hostname <HOSTNAME>
          Sets the hostname to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-routing-port <PORT>
          Sets the port to connect to a custom otoroshi cluster without using a config file
      --otoroshi-user-client-id <CLIENT_ID>
          Sets the client_id to connect to a custom otoroshi cluster without using a config file
      --otoroshi-user-client-secret <CLIENT_SECRET>
          Sets the client_secret to connect to a custom otoroshi cluster without using a config file
      --otoroshi-user-health-key <HEALTH_KEY>
          Sets the health_key to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-cert-location <FILE>
          Sets the client cert location to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-key-location <FILE>
          Sets the client cert key location to connect to a custom otoroshi cluster without using a config file
      --otoroshi-cluster-ca-location <FILE>
          Sets the client cert ca location to connect to a custom otoroshi cluster without using a config file
  -h, --help
          Print help
```