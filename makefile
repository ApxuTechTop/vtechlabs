all:

build:
	docker build -t labs .
run:
	docker run -p80:80 -d labs

deploy:
	kubectl apply -f kubernetes/server-deployment.yaml
	kubectl apply -f kubernetes/server-service.yaml
# проверить работоспособность
	kubectl get pod

# minikube kubectl -- delete services rgr
# minikube kubectl -- delete deployment hello-minikube
# kubectl scale --replicas=4 -f kubernetes/server-deployment.yaml
# kubectl create secret docker-registry myregistrykey --docker-server= --docker-username= --docker-password=
