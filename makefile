all:

build:
	docker build -t labs .
run:
	docker run -p80:80 -d labs
