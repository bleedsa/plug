/ config
cfg:![("cc";   "cxx")
      ("clang";"clang++")]
cfg,:(!).,'("cflags";"-I3rd/clap/include -Wall -Wextra -Wno-unused-parameter")
cfg,:(!).,'("cxxflags";cfg"cflags")

/ ninja
rule:{[n;c]"rule ",n,"\n  command = ",c}
bld:{[R;out;in]"build ",out,": ",R," "," "/in}
cc:bld"cc"
cxx:bld"cxx"

/ files
outof:{"o/",x}
(objof;sharedof):{outof y,".",x}@'("o";"so")

/ filters
ext:{x~|#[x]#|y} / y has ext x?
cpp:ext".cpp"

graphcpp:graph@&cpp'graph:0:"graph"

`0:'("cc = ",cfg"cc"
     "cxx = ",cfg"cxx"
     "cflags = ",cfg"cflags"
     "cxxflags = ",cfg"cxxflags"
     rule["cxx";"$cxx -c $cflags -o $out $in"]
     rule["sharedxx";"$cxx -shared $cflags -o $out $in"]
     "\n"/{cxx[objof x;,"graph/",x]}'graphcpp);
