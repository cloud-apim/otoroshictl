---
sidebar_position: 5
---

# Otoroshi cluster version

at any moment you can get your cluster version with the command

```bash
$ otoroshictl version

+-------------+-------+-------+-------+-------+--------+----------------+
| version     | major | minor | patch | build | suffix | suffix version |
+-------------+-------+-------+-------+-------+--------+----------------+
| 16.17.0-dev | 16    | 17    | 0     |       | dev    |                |
+-------------+-------+-------+-------+-------+--------+----------------+
```

## Version command usage

```bash
$ otoroshictl version -h

Display the version of the current otoroshi cluster

Usage: otoroshictl version [OPTIONS]

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