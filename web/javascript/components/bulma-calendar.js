// Thanks to @megapctr
new Vue({
  el: '#app',
  data() {
    return {
      date: new Date(),
    }
  },
  mounted() {
    const calendar = bulmaCalendar.attach(this.$refs.calendarTrigger, {
      startDate: this.date,
    })[0]
    calendar.on('date:selected', e => (this.date = e.start || null))
  },
  computed: {
    niceDate() {
      if (this.date) {
        return this.date.toLocaleDateString()
      }
    }
  }
});


// The view is like:
<div id='app'>
	Selected date: { { niceDate } }
	<button ref='calendarTrigger' type='button'>Change</button>
</div>