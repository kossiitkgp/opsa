const DateTime = easepick.DateTime;

htmx.onLoad(function () {
    const datePicker = document.getElementById("date-picker");
    const picker = new easepick.create({
        element: datePicker,
        css: [
            "https://cdn.jsdelivr.net/npm/@easepick/bundle@1.2.1/dist/index.css",
            "/assets/date-picker.css",
        ],
        format: "DD MMMM YYYY",
        zIndex: 10,
        plugins: ["AmpPlugin"],
        AmpPlugin: {
            dropdown: {
                months: true,
                years: true,
            },
            darkMode: false,
        },
        setup(picker) {
            picker.on("select", (e) => {
                const channelID = datePicker.getAttribute("data-channel-id");
                const selectedDate = new DateTime(e.detail.date).format(
                    "YYYY-MM-DD HH:mm:ss",
                );
                htmx.ajax(
                    "GET",
                    `/messages/${channelID}?per_page=10&since=${selectedDate}`,
                    {
                        target: ".message-container",
                        swap: "innerHTML",
                    },
                );
                htmx.remove(htmx.find(".thread-container")); // Remove any opened thread
            });
        },
    });
});
