let gulp = require('gulp');
let htmlmin = require('gulp-htmlmin');
let scss = require('gulp-sass');
let minifyCSS = require('gulp-csso');
let uglify = require('gulp-uglify-es').default;
let minify = require('gulp-minify');
let preprocess = require('gulp-preprocess');
let prettyData = require('gulp-pretty-data');
let clean = require('gulp-clean');
let newer = require('gulp-newer');
let ts = require('gulp-typescript');

let tsProject = ts.createProject("tsconfig.json");

let configuredExtensions = [];

gulp.task('html', () => {
	configuredExtensions.push('html');
	return gulp.src([
			'src/**/*.html',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(htmlmin({
			collapseWhitespace: true,
			minifyCSS: true,
			minifyJS: true
		}))
		.pipe(gulp.dest('bin'));
});

gulp.task('scss', () => {
	configuredExtensions.push('sass');
	configuredExtensions.push('scss');
	return gulp.src([
			'src/**/*.{scss,sass}',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(scss())
		.pipe(minifyCSS())
		.pipe(gulp.dest('bin'));
});

gulp.task('css', () => {
	configuredExtensions.push('css');
	return gulp.src([
			'src/**/*.css',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(minifyCSS())
		.pipe(gulp.dest('bin'));
});

gulp.task('js', () => {
	configuredExtensions.push('js');
	return gulp.src([
			'src/**/*.js',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(preprocess())
		.pipe(uglify())
		.pipe(minify({
			ext: {
				min: '.js'
			},
			noSource: true
		}))
		.pipe(gulp.dest('bin'));
});

gulp.task('ts', () => {
	configuredExtensions.push('ts');
	return gulp.src([
			'src/**/*.ts',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(preprocess())
		.pipe(tsProject())
		.pipe(uglify())
		.pipe(minify({
			ext: {
				min: '.js'
			},
			noSource: true
		}))
		.pipe(gulp.dest('bin'));
});

gulp.task('images', () => {
	configuredExtensions.push('png');
	configuredExtensions.push('jpg');
	configuredExtensions.push('jpeg');
	configuredExtensions.push('gif');
	configuredExtensions.push('tif');
	return gulp.src([
			'src/**/*.{png,jpg,jpeg,gif,tif}',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(gulp.dest('bin'));
});

gulp.task('pretty-data', () => {
	configuredExtensions.push('xml');
	configuredExtensions.push('json');
	configuredExtensions.push('xlf');
	configuredExtensions.push('svg');
	return gulp.src([
			'src/**/*.{xml,json,xlf,svg}',
			'!src/**/node_modules/',
			'!src/**/node_modules/**/*'
		])
		.pipe(newer('bin'))
		.pipe(prettyData({
			type: "minify",
			preserveComments: false
		}))
		.pipe(gulp.dest('bin'));
});

gulp.task('copy', () => {
	return gulp.src([
		'src/**/*',
		`!src/**/*.{${configuredExtensions.join(',')}}`,
		'!src/**/node_modules/',
		'!src/**/node_modules/**/*'
	])
	.pipe(newer('bin'))
	.pipe(gulp.dest('bin'));
});

gulp.task('clean', () => {
	return gulp.src('bin')
		.pipe(clean());
});

gulp.task('default', gulp.parallel('html', 'scss', 'css', 'js', 'ts', 'images', 'pretty-data', 'copy'));
gulp.task('build', gulp.series('default'));
gulp.task('rebuild', gulp.series('clean', 'default'));