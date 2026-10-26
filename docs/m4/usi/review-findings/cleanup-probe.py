import sys,time
for l in sys.stdin:
 if l.strip()=="usi": print("id name review-retired-duplicate\nusiok",flush=True)
 elif l.strip()=="isready": print("readyok",flush=True)
 elif l.startswith("go"): print("bestmove 7g7f",flush=True)
 elif l.strip()=="quit":
  time.sleep(0.390)
  print("bestmove 2g2f",flush=True)
  break
