# WCU Course DB

A JSON "database" of all WCU courses as listed on [The course catalog](https://catalog.wcupa.edu/).
Updated weekly.

## How

CourseLeaf (the software WCU uses for its catalog) has XML endpoints for every page. With some
quick regex and finesse we can get all of the info for all courses using this.

## Usage

The endpoint for the database is `https://bwc9876.github.io/wcu_course_db/courses.json`

Simply make a web request with your favorite library or CLI tool (I'll use NuShell below):

```nu
http get https://bwc9876.github.io/wcu_course_db/courses.json
```

...or be boring and use curl or wtv:

```sh
curl https://bwc9876.github.io/wcu_course_db/courses.json
```

See below for structure, here's an example of querying it

```nu
http get https://bwc9876.github.io/wcu_course_db/courses.json | get courses | where code.prefix == "csc" | where distance_available == true | get title
```

(get all Computer Science (CSC) courses with distance education (online sections) available)

## Structure

The general structure of the JSON file is an object with the following fields

### Prefixes

A list of all course prefixes (e.x. eng or csc) (note the casing) that the database contains.

### Courses

A list of course objects that represent all courses found.

#### Course object structure

Fields:

- title (`string`): Human-readable title of the course
- code (`CourseCode`): Object with properties:
  - prefix (`string`): The course prefix (eng, csc, etc) **NOTE THESE WILL BE IN LOWERCASE**
  - number (`number`): The course number
- description (`string`): Description of this course as said in the catalog
- credits (`number`): Number of credits this course counts for
- pre_requirements (`string[]`): A list of pre-reqs for this course (this may or may not be a course, as some require special conditions)
- gen_ed_fulfillments (`string[]`): A list of all gen-ed requirements this course satisfies
- distance_available (`boolean`): Whether this course offer distance education sections (online)
- offered_terms (`string[]`): **usual** terms in which this course is offered, not 100% accurate
