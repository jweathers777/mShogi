solution 'mShogi'
   configurations { 'Debug', 'Release' }

   project 'mShogiEngine'
      kind 'StaticLib'
      language 'C++'
      files { 'src/Engine/**.hpp', 'src/Engine/**.cpp' }
      excludes { 'src/Engine/PieceTypes/DaiDaiShogi/**', 'src/Engine/PieceTypes/ToriShogi/**' }
      includedirs { 'src/Engine', 'src/Engine/**' }

      configuration 'Debug'
         defines {'DEBUG'}
         flags {'Symbols'}

      configuration 'Release'
         defines {'NDEBUG'}
         flags {'Optimize'}

   project 'mShogiConsole'
      kind 'ConsoleApp'
      language 'C++'
      files { 'src/Console/**.hpp', 'src/Console/**.cpp' }
      includedirs { 'src/Console', 'src/Engine', 'src/Engine/**' }
      links { 'mShogiEngine' }

      configuration 'Debug'
         defines {'DEBUG'}
         flags {'Symbols'}

      configuration 'Release'
         defines {'NDEBUG'}
         flags {'Optimize'}
