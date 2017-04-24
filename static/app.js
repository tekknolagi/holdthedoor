Zepto(function($) {
    $.getJSON("/people", function (data) {
        var template = $('#people-template').html();
        var rendered = Mustache.render(template, { people: data });
        $('#people').html(rendered);
    });
});
