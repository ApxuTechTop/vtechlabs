all:

build:
	docker build -t lab1 .
run:
	docker run -p7878:7878 -d lab1
