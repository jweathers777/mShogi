###############################################################################
## Name:         SConstruct
## Description:  SCons construction file
## Created:      05/21/2004 02:14:35 Eastern Standard Time
## Last Updated: $Date: 2010/10/24 13:17:23 $
## Revision:     $Revision: 1.5 $
## Author:       John Weathers
## Email:        hotanguish@hotmail.com
###############################################################################

## ======================================================================
## Import Modules
## ======================================================================

import glob, os, sys, subprocess

## ======================================================================
## Global Settings
## ======================================================================

config_file = 'config.py'
build_name = 'mShogi'

if sys.platform == 'win32':
   boost_path = r'c:\boost'
   wxconfig_cmd = 'wx-config --wxcfg=gcc_lib\msw'
else:
   boost_path = r'/usr/local/include/boost'
   wxconfig_cmd = 'wx-config'

## ======================================================================
## Environment Construction Functions
## ======================================================================

def getBuildOptions():
   # Check for an existing configuration file to use
   if not os.path.exists(config_file):
      # No file exists. Create a configuration file
      print "Creating initial configuration file: " + config_file
      config = file(config_file, 'w')
      config.write("# Configuration file containing user definable options.\n")
      config.write("BOOST = %r\n" % (boost_path))
      config.write("BUILDTYPE = 'release'\n")
      config.close()
      
   build_options = Variables(config_file)

   build_options.AddVariables(
      (  'BOOST',
         'Path to Boost installation',
         boost_path ),
      EnumVariable(
         'BUILDTYPE',
         'Select a release or debug binary.',
         'debug',
         allowed_values = ('release', 'debug','both'),
         map={}, ignorecase=2))
      
   return build_options

def parseCommand(command):
   process = subprocess.Popen(command, shell = True, stdout = subprocess.PIPE, stderr = subprocess.PIPE)
   pid = process.pid
   output, error = process.communicate()
   returncode = process.returncode

   options = output.strip().split()

   return options

## ======================================================================
## Construction Environment Initialization
## ======================================================================

engine_source_paths = Split("""
   Engine Engine/PieceTypes Engine/PieceTypes/ChuShogi
   Engine/PieceTypes/Common Engine/PieceTypes/DaiShogi
   Engine/PieceTypes/Shogi Engine/PieceTypes/TenjikuShogi
   """)

# Create basic environment
env = Environment(options = getBuildOptions())

env['GUI_PROGRAM'] = 'mShogi'         
env['CONSOLE_PROGRAM'] = 'mShogiConsole'
env['ENGINE_LIBRARY'] = 'mShogiEngine'
env['SOURCE_DIR']= 'src'

# Gather list of console program sources
sources = glob.glob(os.path.join(env['SOURCE_DIR'], 'Console', '*.cpp'))
env['CONSOLE_SOURCES'] = \
      [ src.replace(env['SOURCE_DIR'] + os.sep, '') for src in sources ]

# Gather list of GUI program sources
sources = glob.glob(os.path.join(env['SOURCE_DIR'], 'GUI', '*.cpp'))
env['GUI_SOURCES'] = \
      [ src.replace(env['SOURCE_DIR'] + os.sep, '') for src in sources ]

# Gather list of engine library sources
sources = []
for path in engine_source_paths:
   path_elements = [env['SOURCE_DIR']] + path.split('/') + ['*.cpp']
   sources += glob.glob(os.path.join(*path_elements))

env['ENGINE_SOURCES'] = \
      [ src.replace(env['SOURCE_DIR'] + os.sep, '') for src in sources ]

# Parse wxWidgets configuration options
options = parseCommand(wxconfig_cmd + " --cppflags")
env['CPPPATH'] = \
   ['.', env['BOOST']] + \
   [option[2:] for option in options if option[0:2] == '-I'] + \
   ['./'+path for path in engine_source_paths]

env['CPPDEFINES'] = [option[2:] for option in options if option[0:2] == '-D']
env['CXXFLAGS'] = [option for option in options if option[0:2] != '-I' and option[0:2] != '-D']

options = parseCommand(wxconfig_cmd + " --libs")
env['LIBS'] = [option[2:] for option in options if option[0:2] == '-l']
env['LIBPATH'] = ['.'] + [option[2:] for option in options if option[0:2] == '-L']
env['LINKFLAGS'] = [option for option in options if option[0:2] != '-L' and option[0:2] != '-l']

if sys.platform == 'win32':
   env['LINKFLAGS'] = ['--subsystem,windows', '-mwindows'] + env['LINKFLAGS']
   env['RCFLAGS'] = parseCommand(wxconfig_cmd + " --rcflags")
   env['RCINCFLAGS'] = ''
   env['RCFILE'] = build_name + '.rc'
   env['RESFILE'] = build_name + '_rc.o'

# Export our environment for use in child SConscripts
Export('env')

## ======================================================================
## Start Actual Build
## ======================================================================

if env['BUILDTYPE'] in ['both', 'release']:
   # Set up the Sconscript and build directory
   SConscript('release' + os.sep + 'SConscript')
   VariantDir('release', env['SOURCE_DIR'], duplicate=0)

if env['BUILDTYPE'] in ['both', 'debug']:
   # Set up the Sconscript and build directory
   SConscript('debug' + os.sep + 'SConscript')
   VariantDir('debug', env['SOURCE_DIR'], duplicate=0)

