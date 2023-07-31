<script>
  let time;

  function updateClock() {
    const currentTime = new Date();
    const hours = currentTime.getHours();
    const minutes = currentTime.getMinutes();
    const seconds = currentTime.getSeconds();
    const formattedTime = `${hours % 12}:${minutes < 10 ? "0" : ""}${minutes}:${seconds < 10 ? "0" : ""}${seconds} ${hours >= 12 ? "PM" : "AM"}`;

    // Replace "clock" with the ID or class of the element where you want to display the time
    time = formattedTime;
  }
  // Update the clock every second (1000 milliseconds)
  setInterval(updateClock, 1000);
  // Call it once initially to avoid a delay in the first update
  updateClock();
</script>

<!-- mt-3 -->
<div class=" mt-1.5 px-2">
  {new Date().getMonth() + 1 + "/" + new Date().getDate() + "/" + new Date().getFullYear()} -
  {time}
</div>
