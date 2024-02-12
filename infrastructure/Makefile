# Define variables
image_name := default_image
SERVICE = default_service
DOCKERFILE_PATH = ../dockerfiles

# Build target with argument
build:
ifeq ($(SERVICE),default_service)
	@echo "Error: Please specify the Dockerfile name as an argument."
	@echo "Example: make build DOCKERFILE=my_dockerfile"
else ifeq ($(SERVICE),alert_generator_go)
	# Check if Dockerfile exists and assign its path
	image_name := alert-generator-go:latest
	$(DOCKERFILE) = $(shell if test -f "$(DOCKERFILE_PATH)/Dockerfile-$(SERVICE)"; then echo "$(DOCKERFILE_PATH)/Dockerfile-$(SERVICE)"; fi)

	ifeq ($(DOCKERFILE),)
		@echo "Dockerfile not found for service: $(SERVICE)"
	else
		$(CONTAINER_NAME) = alert-generator-go
		i$(IMAGE_NAME) = $(CONTAINER_NAME):latest

		# Build the image
		docker build -t $(IMAGE_NAME) -f $(DOCKERFILE) ../
		docker rm $(CONTAINER_NAME)
		docker run --name $(CONTAINER_NAME) $(IMAGE_NAME)
	endif
else
	@echo "Invalid SERVICE value. Please provide a valid name."
endif

# Other targets (clean, push, run, help) as before
# ...
