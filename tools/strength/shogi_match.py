"""Compatible Shogi entry point for the shared native-game match harness."""
from .native_match import main, summarize, telemetry

__all__ = ["main", "summarize", "telemetry"]

if __name__ == "__main__":
    main("shogi")
