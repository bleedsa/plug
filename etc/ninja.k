/ filters
ext:{x~|#[x]#|y} / y has ext x?
cpp:ext".cpp"
graphcpp:graph@&cpp'graph:0:"graph"

/ config
cfg:!/+((`cc;    "clang")
        (`cxx;   "clang++")
        (`cflags;" "/("-Wall -Wextra -Wno-unused-parameter"
                         "-Wno-unused-variable"
                         "-I3rd/clap/include")))
cfg,:(!).,'(`cxxflags;" "/(cfg`cflags;"-Iinc"))

/ ninja
rule:{[n;c]$[`m~@c;[(cmd;dep):(c@)'`cmd`dep / deconstruct
                    "rule ",n,"\n command = ",cmd,"\n depfile = ",dep]
             "rule ",n,"\n command = ",c]}
bld:{[R;out;in]"build ",out,": ",R," "," "/in}
cc:bld"cc"
cxx:bld"cxx"

/ files
outof:{"o/",x}
(objof;sharedof):{outof y,".",x}@'("o";"so")

shared:bld["sharedxx";sharedof"graph";objof'graphcpp]
outd:(!).,'(`dep;"$out.d")

`0:'("cc = ",cfg`cc
     "cxx = ",cfg`cxx
     "cflags = ",cfg`cflags
     "cxxflags = ",cfg`cxxflags
     rule["exe";"$in"]
     rule["cxx";outd,(!).,'(`cmd;"$cxx -c $cxxflags -MD -MF $out.d -o $out $in")]
     rule["sharedxx";outd,(!).,'(`cmd;"$cxx -shared $cxxflags -o $out $in")]
     "\n"/{cxx[objof x;,"graph/",x]}'graphcpp;shared);
