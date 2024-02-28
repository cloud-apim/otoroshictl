
# DNS
iptables -t nat -N OTOROSHICTL_SIDECAR_DNS_REDIRECT
iptables -t nat -I OUTPUT 1 -p udp --dport 53 -m owner --uid-owner $UID -j RETURN  # si le sidecar veut faire un call dns, on le laisse faire
iptables -t nat -A OUTPUT -p udp --dport 53 -j OTOROSHICTL_SIDECAR_DNS_REDIRECT # si on catch du 53 udp, on va a la regle OTOROSHICTL_SIDECAR_DNS_REDIRECT
iptables -t nat -A OTOROSHICTL_SIDECAR_DNS_REDIRECT -p udp -j REDIRECT --to-ports $DNS_PORT # on redirect en local 53

# OUTBOUND
iptables -t nat -N OTOROSHICTL_SIDECAR_OUTBOUND_REDIRECT
iptables -t nat -I OUTPUT 1 -p tcp --dport 80 -m owner --uid-owner $UID -j RETURN  # si le sidecar veut faire un call dns, on le laisse faire
iptables -t nat -A OUTPUT -p tcp --dport 80 -j OTOROSHICTL_SIDECAR_OUTBOUND_REDIRECT # si on catch du 80 tcp, on va a la regle OTOROSHICTL_SIDECAR_DNS_REDIRECT
iptables -t nat -A OTOROSHICTL_SIDECAR_OUTBOUND_REDIRECT -p tcp -j REDIRECT --to-ports $OUTBOUND_PORT # on redirect en local 80

# INBOUND
iptables -t nat -N OTOROSHICTL_SIDECAR_INBOUND_REDIRECT
iptables -t nat -I OUTPUT 1 -p tcp --dport $TARGET_PORT -m owner --uid-owner $UID -j RETURN  # si le sidecar veut faire un call dns, on le laisse faire
iptables -t nat -A INPUT -p tcp --dport $TARGET_PORT -j OTOROSHICTL_SIDECAR_INBOUND_REDIRECT # si on catch du 80 tcp, on va a la regle OTOROSHICTL_SIDECAR_DNS_REDIRECT
iptables -t nat -A OTOROSHICTL_SIDECAR_INBOUND_REDIRECT -p tcp -j REDIRECT --to-ports $INBOUND_PORT # on redirect en local 80



