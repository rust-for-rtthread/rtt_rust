Import('rtconfig')
from building import *

cwd = GetCurrentDir()
sys.path.append(cwd)

from rust_build import PrebuildRust

src = []
LINKFLAGS = ""

if GetOption('clean'):
  os.system("cd %s; rm -rf rust_out" % cwd)
  group = DefineGroup('rust', src, depend=[])
else:  
  if PrebuildRust(cwd, rtconfig.CPU, Rtt_Root, Rtt_Root+"/../applications/"):
    LINKFLAGS = " -L%s" % (cwd + "/rust_out/")
    LINKFLAGS += " -Wl,--whole-archive -lrust -Wl,--no-whole-archive"
    LINKFLAGS += " -Wl,--allow-multiple-definition"

  group = DefineGroup('rust', src, depend=[], LINKFLAGS=LINKFLAGS)

Return('group')
