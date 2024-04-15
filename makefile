IMAGE_NAME = ghcr.io/arjunkakkar8/lppi-data-hub-api

build:
	docker build -t $(IMAGE_NAME) .

run:
	docker run -it -p 8080:8080 $(IMAGE_NAME) $(ARGS)

push:
	docker push $(IMAGE_NAME)
