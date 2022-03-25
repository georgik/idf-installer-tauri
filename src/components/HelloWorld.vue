<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <h2>Install ESP-IDF</h2>

    <label for="esp-idf-path">Installation directory</label>
    <input id="esp-idf-path" v-model="message">

    <h3>Frameworks</h3>
    <div>
      <ul>
        <li v-for="(version, index) in availableEspIdf" :key="index">
          <input type="checkbox" id="esp-idf-{{ version }}" :value="version" v-model="checkedEspIdf" />
          <label for="esp-idf-{{ version }}">{{ version }}</label>
        </li>
      </ul>

      <div>Selected frameworks: {{ checkedEspIdf }}</div>
    </div>

    <h3>Targets</h3>
    <div>
      <ul>
        <li v-for="(target, index) in availableTargets" :key="index">
          <input type="checkbox" id="target-{{ target }}" :value="target" v-model="checkedTargets" />
          <label for="target-{{ target }}">{{ target }}</label>
        </li>
      </ul>

      <div>Selected targets: {{ checkedTargets }}</div>
    </div>


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
      installButtonTitle: 'Install',
      availableEspIdf: [
        'v4.4',
        'v4.3.2',
        'v4.2.3',
        'v4.1.2',
        'release/v4.4',
        'release/v4.3',
        'release/v4.2',
        'release/v4.1',
        'master'
      ],
      availableTargets: [
        'esp32',
        'esp32s2',
        'esp32s3',
        'esp32c3'
      ],
      checkedEspIdf: [ 'v4.4' ],
      checkedTargets: [ 'esp32', 'esp32s3', 'esp32c3' ],
    }
  },
  props: {
    msg: String,
  },
  mounted() {
    window.__TAURI__.os.platform().then(platform => {
      window.__TAURI__.path.homeDir().then(homeDir => {
        if (platform === "linux") {
          this.message = homeDir + ".espressif"
        } else if (platform === "darwin") {
          this.message = homeDir + ".espressif"
        } else if (platform === "win32") {
          this.message = "C:/Espressif"
        }
      });
    });
  },
  methods: {
    onRustCall: function () {
        console.log('calling tauri');
        this.installButtonTitle = "Installing...";
        window.__TAURI__
          .invoke('deploy_esp_idf_branches_command',{ base:this.message, branches:this.checkedEspIdf })
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
