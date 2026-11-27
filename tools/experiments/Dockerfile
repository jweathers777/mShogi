# Generic cloud worker image: a digest-pinned Python base, the Cloud Storage
# client and prebuilt, verified Linux executables. Workers fetch their own
# source; no engine rebuild or dependency resolution happens on the VM.
ARG PYTHON_BASE
FROM ${PYTHON_BASE}
WORKDIR /opt/mshogi
COPY tools/experiments/requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir -r /tmp/requirements.txt
COPY tools/experiments tools/experiments
COPY tools/strength tools/strength
# build-context/bin holds the engine executables named in the campaign manifest.
COPY build-context/bin /opt/engines
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1
ENTRYPOINT ["python3"]
