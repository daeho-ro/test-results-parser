{% if num_failed > 1 %}### :x: {{ num_failed }} Tests Failed:
| Tests completed | Failed | Passed | Skipped |
|---|---|---|---|
| {{ num_tests }} | {{ num_failed }} | {{ num_passed }} | {{ num_skipped}} |
<details><summary>View the top {{ failures.len() }} failed tests by shortest run time</summary>
{% for failure in failures %}
> 
> ```
> {{ failure.test_name }}
> ```
> 
> <details><summary>Stack Traces | {{ failure.duration }}s run time</summary>
> 
> > {{ failure.backticks }}{% for stack_trace_line in failure.stack_trace %}
> > {{ stack_trace_line }}{% endfor %}
> > {{ failure.backticks }}{% match failure.build_url %}{% when Some with (build_url) %}
> > [View]({{ build_url }}) the CI Build{% when None %}{% endmatch %}
> 
> </details>

{% endfor %}
</details>{% else %}:white_check_mark: All tests successful. No failed tests were found.{% endif %}

:mega: Thoughts on this report? [Let Codecov know!](https://github.com/codecov/feedback/issues/304) | Powered by [Codecov](https://about.codecov.io/)
