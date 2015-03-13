solution 'mShogi'
   configurations { 'Debug', 'Release' }

   configuration 'Debug'
      defines {'DEBUG'}
      flags {'Symbols'}
      buildoptions { "`wx-config --cxxflags`", "-ansi", "-pedantic", "-Wno-c++11-long-long", "-Wno-variadic-macros" } 
      linkoptions { "`wx-config --libs`" }

   configuration 'Release'
      defines {'NDEBUG'}
      flags {'Optimize'}
      buildoptions { "`wx-config --cxxflags`", "-ansi", "-pedantic", "-Wno-c++11-long-long", "-Wno-variadic-macros" } 
      linkoptions { "`wx-config --libs`" }

   project 'mShogiEngine'
      kind 'StaticLib'
      language 'C++'
      files { 'src/Engine/**.hpp', 'src/Engine/**.cpp' }
      excludes { 'src/Engine/PieceTypes/DaiDaiShogi/**', 'src/Engine/PieceTypes/ToriShogi/**' }
      includedirs { 'src/Engine', 'src/Engine/**' }


   project 'mShogiConsole'
      kind 'ConsoleApp'
      language 'C++'
      files { 'src/Console/**.hpp', 'src/Console/**.cpp' }
      includedirs { 'src/Console', 'src/Engine', 'src/Engine/**' }
      links { 'mShogiEngine' }

   project 'mShogiGui'
      kind 'WindowedApp'
      language 'C++'
      files { 'src/GUI/**.hpp', 'src/GUI/**.cpp' }
      includedirs { 'src/GUI', 'src/Engine', 'src/Engine/**' }
      links { 'mShogiEngine' }

