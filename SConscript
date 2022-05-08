Import('rtconfig')
from building import *

cwd = GetCurrentDir()
sys.path.append(cwd)

from rust_build import PrebuildRust
from rust_build import SeleceFeature
from rust_build import PrepareSetFeature
from rust_build import ClearFeature

src = []
LINKFLAGS = ""

if GetOption('clean'):
  ClearFeature(cwd)
  os.system("cd %s; rm -rf rust_out" % cwd)
  group = DefineGroup('rust', src, depend=[])
else:  
  PrepareSetFeature(cwd)
  if GetDepend("RT_USING_SMP"):
    SeleceFeature("smp")

  ret = PrebuildRust(cwd, rtconfig.CPU, Rtt_Root, Rtt_Root+"/../applications/")
  if ret == "OK":
    LINKFLAGS = " -L%s" % (cwd + "/rust_out/")
    LINKFLAGS += " -Wl,--whole-archive -lrust -Wl,--no-whole-archive"
    LINKFLAGS += " -Wl,--allow-multiple-definition"
  elif ret == "PASS":
    pass
  elif ret == "ERR":
    raise Exception("RUST BUILD FATAL ERROR!!!")
    
  group = DefineGroup('rust', src, depend=[], LINKFLAGS=LINKFLAGS)

Return('group')
