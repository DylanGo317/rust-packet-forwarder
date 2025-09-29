
Build docker image
`docker build -t dylanpwgo/l3_forwarder:latest .`

Run docker image and open bash
`docker run --rm -it dylanpwgo/l3_forwarder:latest bash`

Apply deployment.yaml to the kubectl deployment
`kubectl apply -f deployment.yaml`

Open a terminal insde the pod
`kubectl exec -it <pod_name> -- bash`

View stdout as it is being printed inside a pod
`kubectl logs -f <pod_name>`

Find IP address of a pod
`kubectl get pods -o wide`

Forwards your localhost:80 to pod port 8080.
kubectl port-forward pod/<pod-name> 80:8080
