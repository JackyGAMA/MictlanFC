FROM rust:1.84-bookworm

# Instalar dependencias del sistema, Python y librerías nativas requeridas por stellar-cli
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    curl \
    tar \
    git \
    libdbus-1-3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/

# Instalar target de Soroban para Rust
RUN rustup target add wasm32v1-none

# Instalar stellar-cli precompilado (v28.0.0) para Linux x86_64
RUN curl -L https://github.com/stellar/stellar-cli/releases/download/v28.0.0/stellar-cli-28.0.0-x86_64-unknown-linux-gnu.tar.gz -o stellar.tar.gz \
    && tar -xzf stellar.tar.gz -C /usr/local/bin/ \
    && rm stellar.tar.gz \
    && stellar --version

# Configurar directorio de trabajo
WORKDIR /app

# Instalar dependencias de Python
COPY requirements.txt .
RUN pip3 install --no-cache-dir --break-system-packages -r requirements.txt

# Copiar el código del proyecto
COPY . .

# Precompilar dependencias de Soroban para que los deploys y tests sean ultrarrápidos
RUN cd contracts/hello-world && cargo check --target wasm32v1-none && cargo test --no-run || true

# Configurar variables de entorno
ENV PORT=8000
ENV PYTHONUNBUFFERED=1

EXPOSE 8000

# Iniciar servidor FastAPI
CMD ["sh", "-c", "uvicorn main:app --host 0.0.0.0 --port ${PORT:-8000}"]
