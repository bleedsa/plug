#include <u.h>
#include <vec.h>

#include <stdlib.h>
#include <stdio.h>

#include <clap/clap.h>

struct Voice {
	bool held;
	i32 note_id;
	i16 chan, key;
	float phase;
};

struct Graph {
	Plug plugin;
	con Host *host;
	float sample_rate;
	B<Voice> voices;
};

sta X mkgraph(con Host*h)->Graph*{
	X G=(Graph*)calloc(1,Z(Graph));
	G->host=h;
	G->plugin.plugin_data=G;
	return G;
}

sta con PlugDesc plug_desc={
	.clap_version=CLAP_VERSION_INIT,
	.id="BSA.Graph",
	.name="BSA Graph",
	.vendor="bleedsa",
	.url="https://badboy.institute/~skye",
	.manual_url="",
	.support_url="",
	.version="1.0.0",
	.description="k graphing calculator synth",
	.features=(CC*[]){
		CLAP_PLUGIN_FEATURE_INSTRUMENT,
		CLAP_PLUGIN_FEATURE_SYNTHESIZER,
		CLAP_PLUGIN_FEATURE_STEREO,
		nullptr,
	},
};

sta con Plug plug_class={
	.desc=&plug_desc,
	.plugin_data=nullptr,
	.init=[](con Plug*x)->bool{X p=(Graph*)x->plugin_data;return true;},
	.destroy=[](con Plug*x){
		X p=(Graph*)x->plugin_data;
		p->voices.del();
		return true;
	},
};

sta con Factory factory={
	.get_plugin_count=[](con Factory*f)->u32{
		return 1;
	},
	.get_plugin_descriptor=[](con Factory*f,u32 i)->con PlugDesc*{
		return i == 0 ? &plug_desc : nullptr;
	},
	.create_plugin=[](con Factory *f,con Host*h,CC*id)->con Plug*{
		X G=mkgraph(h);
		return&G->plugin;
	},
};

extern "C" con clap_plugin_entry_t clap_entry={
	.clap_version=CLAP_VERSION_INIT,
	.init=[](CC*path)->bool{return true;},
	.deinit=[](){},
	.get_factory=[](CC *id)->con V*{
		return strcmp(id,CLAP_PLUGIN_FACTORY_ID)?nullptr:&factory;
	},
};
