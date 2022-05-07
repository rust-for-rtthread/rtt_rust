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

  if PrebuildRust(cwd, rtconfig.CPU, Rtt_Root, Rtt_Root+"/../applications/"):
    LINKFLAGS = " -L%s" % (cwd + "/rust_out/")
    LINKFLAGS += " -Wl,--whole-archive -lrust -Wl,--no-whole-archive"
    LINKFLAGS += " -Wl,--allow-multiple-definition"

  group = DefineGroup('rust', src, depend=[], LINKFLAGS=LINKFLAGS)

Return('group')
