# Use Python 3.13 slim image
FROM python:3.13-slim

# Set working directory
WORKDIR /app

# Copy project files
COPY pyproject.toml .
COPY source/ source/
COPY server.py .
COPY __init__.py .

# Install dependencies
RUN pip install --no-cache-dir .

# Build documentation
RUN sphinx-build -b dirhtml source dist/dirhtml

# Expose port 80
EXPOSE 80

# Run the server
CMD ["python", "server.py"]
