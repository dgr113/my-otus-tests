# Схема:
![Project schema](./_doc/kafka-otus-schema.jpg)


# Установка

## Docker-сompose

```shell
/usr/bin/kafka-topics --create --partitions 2 --replication-factor 1 --zookeeper zoo1:2181 --topic order-topic;
/usr/bin/kafka-topics --create --partitions 2 --replication-factor 1 --zookeeper zoo1:2181 --topic billing-topic;
/usr/bin/kafka-topics --create --partitions 2 --replication-factor 1 --zookeeper zoo1:2181 --topic notify-topic;
```

## Kubernetes:

Добавляем helm репозиторий и устанавливаем чарт:
```shell
helm repo add confluentinc https://confluentinc.github.io/cp-helm-charts/;
helm repo update;
helm install cp-app confluentinc/cp-helm-charts -f ./_charts/confluent/cp-values.yaml;
```

Запускаем Pod с клиентом под Kafka и затем создаем наши топики через этот интерфейс:
```shell
kubectl apply -f ./_charts/confluent/kafka-client.yaml;
kubectl exec kafka-client -- kafka-topics --zookeeper cp-app-cp-zookeeper:2181 --topic order-topic --create --partitions 2 --replication-factor 1;
kubectl exec kafka-client -- kafka-topics --zookeeper cp-app-cp-zookeeper:2181 --topic billing-topic --create --partitions 2 --replication-factor 1;
kubectl exec kafka-client -- kafka-topics --zookeeper cp-app-cp-zookeeper:2181 --topic notify-topic --create --partitions 2 --replication-factor 1;
```

Устанавливаем чарт с нашими сервисами:
```shell
helm install kfk-test-app ./_charts/kafka-app;
```


## Особенности установки:
Временное решение [проблемы](https://github.com/kubernetes/minikube/issues/11121) с Ingress в Minikube  
(некоторые версии):
```shell
kubectl delete -A ValidatingWebhookConfiguration ingress-nginx-admission;
```


## Тесты:
[Postman](./_tests/99_kafta_test.postman_collection.json)
