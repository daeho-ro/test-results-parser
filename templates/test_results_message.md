### :x: {{ num_failed }} Tests Failed:
| Tests completed | Failed | Passed | Skipped |
|---|---|---|---|
| {{ num_tests }} | {{ num_failed }} | {{ num_passed }} | {{ num_skipped}} |
<details><summary>View the top {{ num_output }} failed tests by shortest run time</summary>
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
> > {{ failure.backticks }}{% if failure.build_url %}
> > [View]({{ failure.build_url }}) the CI Build{% endif %}
> 
> </details>

{% endfor %}