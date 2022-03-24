<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h2>Install ESP-IDF</h2>
    <input id="esp-idf-path" v-model="message">
    <button v-on:click="onRustCall()">
      {{ installButtonTitle }}
    </button>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  data() {
    return {
      message: 'C:/esp-idf-master',
      installButtonTitle: 'Install'
    }
  },
  props: {
    msg: String,
  },
  mounted() {
    window.__TAURI__.os.platform().then(platform => {
      window.__TAURI__.path.homeDir().then(homeDir => {
        if (platform === "linux") {
          this.message = homeDir + ".espressif/frameworks/esp-idf-master"
        } else if (platform === "darwin") {
          this.message = homeDir + ".espressif/frameworks/esp-idf-master"
        } else if (platform === "win32") {
          this.message = "C:/Espressif/frameworks/esp-idf-master"
        }
      });
    });
  },
  methods: {
    onRustCall: function () {
        console.log('calling tauri');
        this.installButtonTitle = "Installing...";
        window.__TAURI__
          .invoke('simple_command',{argument:this.message})
          .then((response) => {
            this.installButtonTitle = "Finished";
            console.log('ok' + response)
            
          })
          .catch((error) => {
            this.installButtonTitle = "Failed";
            console.log('fail' + error)
          })
      
    }
  },
  mutations: {
    increment (state)  {
      state.msg = 'abc';
    }
  }

}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
#esp-idf-path {
  width: 25em;
}
</style>
