  new Vue({
    el: "#app",
    data: {
      showNotificationBox: false
    },
    methods: {
      toggleNotifCompView: function() {
        console.log('calling tNCV');
        this.showNotificationBox = !this.showNotificationBox;
      }
    }
  })
