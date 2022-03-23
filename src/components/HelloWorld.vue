<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h2>Install ESP-IDF</h2>
    <input v-model="message" placeholder="C:/Espressif/frameworks/esp-idf-master">
    <p>Message is: {{ message }}</p>
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
</style>
