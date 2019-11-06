Vue.component('notification-box', {
    data: function() {
        return {
            // notifications: {}
            notifications: {
                1: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Mr. Fuji'
                },
                2: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Yoshi Tatsu'
                },
                3: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Shinsuke Nakamura'
                },
                4: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Funaki'
                },
                5: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Asuka'
                },
                6: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Hideo Itami'
                },
                7: {
                    time: new Date(),
                    body: 'This is a notification body',
                    from: 'Tetsuya Naito'
                }
            }
        }
    },
    template: `
        <section class="notificationComponent">
            <div class='notif-container'>
                <header class="notif-cont-header">
                    <div class="notif-recent">
                        <span>Recent ({{Object.keys(notifications).length}})</span>
                    </div>
                </header>
                <div class="no-notifs" v-if="Object.keys(notifications).length == 0">
                    <span>No new notifications</span>
                </div>
                <div class="notif" v-else v-for="notification in notifications">
                    <div class="user-icon">
                        <img width=50 height=50 src="https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/intermediary/f/6b30ad28-e606-4523-a2d2-a13b8569c3b2/d5yilwu-ef2fe22b-85a7-4863-8f2b-41df80edb1bc.jpg">
                    </div>
                     <div class="notif-content">
                        <div class="notif-subject">
                            <span><a href="#">{{notification.from}}</a> sent you a notification</span>
                        </div>
                        <div class="notif-body">
                            <span>{{notification.body}}</span>
                        </div>
                        <div class="notif-time">
                            <span>{{notification.time}}</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    `
});