Zepto(function($) {
    $.getJSON("/db", function (data) {
        var template = $('#people-template').html();
        var rendered = Mustache.render(template, data);
        $('#people').html(rendered);
    });
});
