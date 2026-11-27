"""Shared fixture selection and event records for desktop verification tools."""

from datetime import datetime, timezone
import threading
import time

# External startup names supported by the separately labeled QA applications.
CHESS_FIXTURES = ("standard", "promotion", "claim", "current-claim", "concluded", "capacity")
SHOGI_FIXTURES = ("shogi-hands", "shogi-promotion", "shogi-forced", "shogi-declaration",
                  "shogi-concluded", "shogi-capacity")
XIANGQI_FIXTURES = ("xiangqi-warning", "xiangqi-fourth", "xiangqi-count", "xiangqi-offer",
                    "xiangqi-agreed", "xiangqi-fast-proof", "xiangqi-resigned", "xiangqi-capacity",
                    "xiangqi-failure")
TENJIKU_PROFILES = ("adams", "muller")
QA_FIXTURES = frozenset((
    *CHESS_FIXTURES, *SHOGI_FIXTURES, *XIANGQI_FIXTURES,
    "shared-contract", "incomplete", "shogi-standard", "shogi-gote", "shogi-failure",
    "shogi-control-capacity", "xiangqi-standard", "xiangqi-control-capacity",
    "chu-standard", "chu-gote", "chu-contract", "chu-promoted", "chu-royals",
    "chu-captures", "chu-captured", "chu-promotion", "tori-standard", "tori-contract",
    *(f"tenjiku-{profile}-{scenario}"
      for profile in TENJIKU_PROFILES
      for scenario in ("standard", "gote", "compound", "compound-played", "selected-root",
                       "burn", "burned", "igui", "iguied", "promotion", "promoted", "declined",
                       "hawk", "hawk-played", "promoted-hawk", "area", "area-played",
                       "ranked-jump", "ranked-jump-played", "ranked-blocked", "incomplete",
                       "ending-stopped", "all-faces", "engine-played", "analysis")),
))
SYNTHETIC_FIXTURE = "synthetic"


def validate_fixture(fixture, *, synthetic=False):
    valid = fixture == SYNTHETIC_FIXTURE if synthetic else fixture in QA_FIXTURES
    if not valid or len(fixture.encode("utf-8")) + 1 > 128:
        raise ValueError("Unsupported fixture for this fixed QA identity")


def stamp():
    return datetime.now(timezone.utc).isoformat()


def event_record(operation, observation):
    return {"at_utc": stamp(), "operation": operation, "observation": observation}


class NativeEventLog:
    def __init__(self):
        self.events = []
        self._event_lock = threading.Lock()
        self._started = time.monotonic()

    def _event(self, operation, **observation):
        with self._event_lock:
            observation["elapsed_ms"] = round((time.monotonic() - self._started) * 1000)
            self.events.append(event_record(operation, observation))
        return observation
