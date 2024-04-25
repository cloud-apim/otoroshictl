---
sidebar_position: 1
---

# Cloud APIM integration

you can link your [Cloud APIM](https://www.cloud-apim.com) account into your `otoroshictl` config, just do

```bash 
$ otoroshictl cloud-apim login
```

it should open a web browser where you can log into your [Cloud APIM](https://www.cloud-apim.com) account. Once logged in, you be able to list your deployments


```bash
$ otoroshictl cloud-apim list

+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+
| name                                    | kind     | version  | status  | region | plan               | created_at               |
+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+
| wasi-wasm-demo                          | Otoroshi | v16.16.1 | Running | par    | xxxxxxxxxxxxxxxxxx | 2023-11-13T13:55:49.741Z |
+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+
| balanced analyzing budgetary management | Otoroshi | v16.16.1 | Running | par    | xxxxxxxxxxxxxxxxxx | 2024-03-07T09:55:59.424Z |
+-----------------------------------------+----------+----------+---------+--------+--------------------+--------------------------+
```

and then link a deployement to your `otoroshictl` config

```bash
$ otoroshictl cloud-apim link wasi-wasm-demo
```

## Cloud APIM subcommands

```bash
$ otoroshictl cloud-apim -h

Manage cloud apim clusters

Usage: otoroshictl cloud-apim [OPTIONS] <COMMAND>

Commands:
  login    Login to your cloud-apim account
  list     List your deployments
  logout   Logout from your cloud-apim account
  link     Add the cluster to the possible otoroshictl configs
  use      Add the cluster to the possible otoroshictl configs and set it as the current one
  restart  Restart this otoroshi cluster on cloud-apim
  help     Print this message or the help of the given subcommand(s)

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