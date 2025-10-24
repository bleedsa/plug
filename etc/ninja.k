/ filters
ext:{x~|#[x]#|y} / y has ext x?
cpp:ext".cpp"
graphcpp:graph@&cpp'graph:0:"graph"

/ config
cfg:(!).+(("cc";"clang")
          ("cxx";"clang++")
          ("cflags";"-Wall -Wextra -Wno-unused-parameter -I3rd/clap/include"))
cfg,:(!).,'("cxxflags";" "/(cfg"cflags";"-Iinc"))

/ ninja
rule:{[n;c]"rule ",n,"\n  command = ",c}
bld:{[R;out;in]"build ",out,": ",R," "," "/in}
cc:bld"cc"
cxx:bld"cxx"

/ files
outof:{"o/",x}
(objof;sharedof):{outof y,".",x}@'("o";"so")

shared:bld["sharedxx";sharedof"graph";objof'graphcpp]

`0:'("cc = ",cfg"cc"
     "cxx = ",cfg"cxx"
     "cflags = ",cfg"cflags"
     "cxxflags = ",cfg"cxxflags"
     rule["exe";"$in"]
     rule["cxx";"$cxx -c $cxxflags -MD -MF $out.d -o $out $in"]
     rule["sharedxx";"$cxx -shared $cxxflags -o $out $in"]
     "\n"/{cxx[objof x;,"graph/",x]}'graphcpp;shared);
