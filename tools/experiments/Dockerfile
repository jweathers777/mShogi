# Supply digest-pinned base and prebuilt, verified Linux executables; no engine
# rebuild or network dependency resolution occurs when a worker starts.
ARG PYTHON_BASE
FROM ${PYTHON_BASE}
WORKDIR /opt/mshogi
COPY tools/experiments/requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir -r /tmp/requirements.txt
COPY tools/experiments tools/experiments
COPY docs/m5/evidence/msh161 docs/m5/evidence/msh161
COPY docs/m5/competitive-gameplay-diagnosis.md docs/m5/competitive-gameplay-diagnosis.md
# build-context/bin must contain mshogi, mshogi-match, oxidation and stockfish.
# Build provenance and hashes are in the separately frozen job manifest.
COPY build-context/bin /opt/engines
ENV PYTHONDONTWRITEBYTECODE=1 PYTHONUNBUFFERED=1
ENTRYPOINT ["python3", "-m", "tools.experiments.cli"]
