command: "unreadmail.widget/gmail_unread_counter/target/release/mail_unread_counter -t='unreadmail.widget/tokencache.json' -c='unreadmail.widget/credentials.json'
 | tail -2 | sed 's/\"//g' | sed 's/@.*//g'"

refreshFrequency: 60000

style: """
  top: -2%
  left: 28%
  color: #bb0
  bakground-color: red
  font-family: Helvetica
  padding-top: 24px
  padding-right: 26px

  span
    color: #fff
    background-color: #a30404
    height: 22px
    width: 22px
    border-radius: 50%
    font-size: 14px
    line-height: 22px
    text-align: center
    position: absolute
    top: 20px
    right: 14px
    z-index: 1

  .email
    color: black
    font-size: 15px
    line-height: 50px
    text-align: center
    position: absolute
    top: 50%
    left: 1%
  svg
    transform: scale(1.1)
"""


render: -> """
<span class='mail-count'></span>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Capa_1" x="0px" y="0px" width="50px" height="50px" viewBox="0 0 511.626 511.626" style="enable-background:new 0 0 511.626 511.626;" xml:space="preserve"><g><g>
	<g>
		<path d="M49.106,178.729c6.472,4.567,25.981,18.131,58.528,40.685c32.548,22.554,57.482,39.92,74.803,52.099    c1.903,1.335,5.946,4.237,12.131,8.71c6.186,4.476,11.326,8.093,15.416,10.852c4.093,2.758,9.041,5.852,14.849,9.277    c5.806,3.422,11.279,5.996,16.418,7.7c5.14,1.718,9.898,2.569,14.275,2.569h0.287h0.288c4.377,0,9.137-0.852,14.277-2.569    c5.137-1.704,10.615-4.281,16.416-7.7c5.804-3.429,10.752-6.52,14.845-9.277c4.093-2.759,9.229-6.376,15.417-10.852    c6.184-4.477,10.232-7.375,12.135-8.71c17.508-12.179,62.051-43.11,133.615-92.79c13.894-9.703,25.502-21.411,34.827-35.116    c9.332-13.699,13.993-28.07,13.993-43.105c0-12.564-4.523-23.319-13.565-32.264c-9.041-8.947-19.749-13.418-32.117-13.418H45.679    c-14.655,0-25.933,4.948-33.832,14.844C3.949,79.562,0,91.934,0,106.779c0,11.991,5.236,24.985,15.703,38.974    C26.169,159.743,37.307,170.736,49.106,178.729z" data-original="#000000" class="active-path" data-old_color="#000000" fill="#cecece"/>
		<path d="M483.072,209.275c-62.424,42.251-109.824,75.087-142.177,98.501c-10.849,7.991-19.65,14.229-26.409,18.699    c-6.759,4.473-15.748,9.041-26.98,13.702c-11.228,4.668-21.692,6.995-31.401,6.995h-0.291h-0.287    c-9.707,0-20.177-2.327-31.405-6.995c-11.228-4.661-20.223-9.229-26.98-13.702c-6.755-4.47-15.559-10.708-26.407-18.699    c-25.697-18.842-72.995-51.68-141.896-98.501C17.987,202.047,8.375,193.762,0,184.437v226.685c0,12.57,4.471,23.319,13.418,32.265    c8.945,8.949,19.701,13.422,32.264,13.422h420.266c12.56,0,23.315-4.473,32.261-13.422c8.949-8.949,13.418-19.694,13.418-32.265    V184.437C503.441,193.569,493.927,201.854,483.072,209.275z" data-original="#000000" class="active-path" data-old_color="#000000" fill="#cecece"/>
	</g>
<div class="email"></div>
</g></g> </svg>
"""

update: (numberOfUnreadMails, domEl) ->
  lines = numberOfUnreadMails.split '\n'
  $(domEl).find('.email').html(" #{lines[1]}")
  $(domEl).find('.mail-count').html(" #{lines[0]}")
