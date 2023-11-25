
import os
import sys
import shutil

# "prebuild":  "(mkdir .\\dist\\pp || true) && (mkdir .\\backup\\x\\pp || true) && copy .\\dist\\pp .\\backup\\x\\pp",
# "postbuild": "(mkdir .\\dist\\pp || true) && (mkdir .\\backup\\x\\pp || true) && move .\\backup\\x\\pp .\\dist\\pp"

# os.getcwd()

dist_path = ".\\dist\\imgs\\pp"
backup_path = ".\\backup\\x\\pp"

if (len(sys.argv) < 2):
  print('ERR: no arg')
elif (sys.argv[1] == 'pre'):
    if os.path.isdir(dist_path):
      shutil.move(dist_path, backup_path)
elif (sys.argv[1] == 'post'):
    if os.path.isdir(backup_path):
      shutil.move(backup_path, dist_path)
else:
  print('ERR: unkn param value', sys.argv[1])