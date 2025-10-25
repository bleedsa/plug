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

struct Tmp {
	Plug plugin;
	con Host*host;
	float sample_rate;
	B<Voice> voices;
};

inl sta X getplug(con Plug*x)->Tmp*{return (Tmp*)x->plugin_data;}
inl sta X delvoices(con Plug*x)->V{X P=getplug(x);P->voices.del();}

sta con PlugDesc plug_desc={
	.clap_version=CLAP_VERSION_INIT,
	.id="BSA.Tmp",
	.name="BSA Tmp",
	.vendor="bleedsa",
	.url="https://badboy.institute/~skye",
	.manual_url="",
	.support_url="",
	.version="1.0.0",
	.description="tmp",
	.features=(CC*[]){
		CLAP_PLUGIN_FEATURE_INSTRUMENT,
		CLAP_PLUGIN_FEATURE_SYNTHESIZER,
		CLAP_PLUGIN_FEATURE_STEREO,
		nullptr,
	},
};

sta con NotePorts note_ports={
	.count=[](con Plug*x,bool inp)->u32{return inp?1:0;},
	.get=[](con Plug*x,u32 i,bool inp, NotePortInfo*info)->bool{
		if(!inp||i)return false;
		info->id=0,info->supported_dialects=CLAP_NOTE_DIALECT_CLAP;
		info->preferred_dialect=CLAP_NOTE_DIALECT_CLAP;
		snprintf(info->name,Z(info->name),"%s","Note Port");
		return true;
	},
};

sta con AudioPorts audio_ports={
	.count=[](con Plug*x,bool inp)->u32{return inp?0:1;},
	.get=[](con Plug*x,u32 idx,bool inp,AudioPortInfo*info)->bool{
		if(inp||idx)return false;
		info->id=0,info->channel_count=2;
		info->flags=CLAP_AUDIO_PORT_IS_MAIN;
		info->port_type=CLAP_PORT_STEREO;
		info->in_place_pair=CLAP_INVALID_ID;
		snprintf(info->name,Z(info->name),"%s","Audio Output");
		return true;
	},
};

sta con Plug plug_class={
	.desc=&plug_desc,
	.plugin_data=nullptr,
	.init=[](con Plug*x)->bool{X P=(Tmp*)x->plugin_data;return true;},
	.destroy=[](con Plug*x){delvoices(x);free((V*)x);},
	.activate=[](con Plug*x,f64 rate,u32 minframe,u32 maxframe)->bool{
		X P=getplug(x);P->sample_rate=rate;
		return true;
	},
	.deactivate=[](con Plug*x){},
	.start_processing=[](con Plug*x){return true;},
	.reset=[](con Plug*x){delvoices(x);},
	.process=[](con Plug*x,con Process*p)->clap_process_status{
		X P=getplug(x);
		return CLAP_PROCESS_CONTINUE;
	},
	.get_extension=[](con Plug*x,CC*id)->con V*{
		if(streq(id,CLAP_EXT_NOTE_PORTS))return&note_ports;
		if(streq(id,CLAP_EXT_AUDIO_PORTS))return&audio_ports;
		return nullptr;
	},
};

sta X mktmp(con Host*h)->Tmp*{
	X P=(Tmp*)calloc(1,Z(Tmp));
	P->host=h;
	P->plugin=plug_class;
	P->plugin.plugin_data=P;
	return P;
}


sta con Factory factory={
	.get_plugin_count=[](con Factory*f)->u32{
		return 1;
	},
	.get_plugin_descriptor=[](con Factory*f,u32 i)->con PlugDesc*{
		return i == 0 ? &plug_desc : nullptr;
	},
	.create_plugin=[](con Factory *f,con Host*h,CC*id)->con Plug*{
		X P=mktmp(h);
		return&P->plugin;
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
