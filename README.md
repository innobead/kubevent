> Note: This is an early and experimental project. Please don't use in production and wait for following updates in the future.

# What is Kubevent? 

**Kubevent**, a K8s event telemetry solution to filter, convert and transfer CloudEvents compatible K8s events to external supported event brokers.

- Monitors interested K8s events to convert to CloudEvents events.
- Supports rule configurations for different type of event filtering and enrichment.
- Supports to transfer transformed CloudEvents events to external event brokers.

## Getting Started

There will be a CLI to support `kubevent` installation and management, but it's under development. For now, please build and try directly by using below commands.


### Applying CRDs (at least K8s 1.18)
```console
kubectl apply -f ./manifests/generated
```

### Starting kubeventd
```console
make release
./target/release/kubeventd
```

### Applying sample configurations
```console
kubectl apply -f ./manifests/sample.yaml
```

## Supported CRDs to control event processing

### Rule
```yaml
apiVersion: kubevent.io/v1alpha1
kind: Rule
metadata:
  name: demo
spec:
  kind: "type"
  types: [ "Warning" ]
```

### Broker
```
apiVersion: kubevent.io/v1alpha1
kind: Broker
metadata:
  name: demo
spec:
  kind: "console"
```

### RuleBrokersBinding
```
apiVersion: kubevent.io/v1alpha1
kind: RuleBrokersBinding
metadata:
  name: demo
spec:
  rule: "demo"
  brokers: [ "demo" ]
```

## Supported Rules

- **type**: filters events by K8s event type
- ... WIP

## Supported Brokers

- **console**: sends events to the console
- ... WIP

## Demos

[![asciicast](https://asciinema.org/a/8VZllAkQjcXPYHbD3oJZcdcfX.svg)](https://asciinema.org/a/8VZllAkQjcXPYHbD3oJZcdcfX)

# References
- https://cloudevents.io/
- https://github.com/cloudevents/spec
